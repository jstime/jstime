use jstime_core as jstime;
use rustc_hash::{FxHashMap, FxHashSet};
use rustyline::Helper;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::{ValidationContext, ValidationResult, Validator};
use rustyline::{
    Context,
    completion::{Completer, Pair},
};
use std::sync::{Arc, RwLock};

/// Cached completion data for the REPL.
/// This is populated at startup and refreshed after each command.
pub(crate) struct CompletionCache {
    /// Global names (properties of globalThis) plus tracked REPL lexical bindings.
    pub(crate) globals: Vec<String>,
    /// Cached property names for known objects (lazily populated)
    pub(crate) properties: FxHashMap<String, Vec<String>>,
}

impl CompletionCache {
    pub(crate) fn new() -> Self {
        CompletionCache {
            globals: Vec::new(),
            properties: FxHashMap::default(),
        }
    }
}

// JavaScript keywords that should always be available for completion
const JS_KEYWORDS: &[&str] = &[
    "const",
    "let",
    "var",
    "function",
    "return",
    "if",
    "else",
    "for",
    "while",
    "break",
    "continue",
    "switch",
    "case",
    "default",
    "try",
    "catch",
    "finally",
    "throw",
    "new",
    "this",
    "typeof",
    "instanceof",
    "in",
    "of",
    "delete",
    "void",
    "async",
    "await",
    "class",
    "extends",
    "static",
    "import",
    "export",
    "from",
    "true",
    "false",
    "null",
    "undefined",
];

// Global values that don't have meaningful properties for completion
const JS_PRIMITIVES: &[&str] = &["undefined", "NaN", "Infinity"];

/// JavaScript completer for REPL that uses dynamically discovered completions.
pub(crate) struct JsCompleter {
    cache: Arc<RwLock<CompletionCache>>,
}

impl JsCompleter {
    pub(crate) fn new(cache: Arc<RwLock<CompletionCache>>) -> Self {
        JsCompleter { cache }
    }
}

impl Completer for JsCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        // Check if we're completing a property (after a dot)
        if let Some(dot_pos) = line[..pos].rfind('.') {
            // Get the object expression before the dot
            let before_dot = &line[..dot_pos];
            let obj_start = before_dot
                .rfind(|c: char| !is_completion_identifier_char(c))
                .map(|i| i + 1)
                .unwrap_or(0);
            let obj_expr = &before_dot[obj_start..];

            // Get the property prefix after the dot
            let property_start = dot_pos + 1;
            let property_prefix = &line[property_start..pos];

            // Look up cached properties for this object
            let cache = self.cache.read().unwrap();
            let properties = cache.properties.get(obj_expr);

            let mut completions: Vec<Pair> = if let Some(props) = properties {
                props
                    .iter()
                    .filter(|p| p.starts_with(property_prefix))
                    .map(|p| Pair {
                        display: p.to_string(),
                        replacement: p.to_string(),
                    })
                    .collect()
            } else {
                Vec::new()
            };

            completions.sort_by(|a, b| a.display.cmp(&b.display));

            return Ok((property_start, completions));
        }

        // Regular keyword/global completion (no dot)
        let start = line[..pos]
            .rfind(|c: char| !is_completion_identifier_char(c))
            .map(|i| i + 1)
            .unwrap_or(0);

        let word = &line[start..pos];

        // Combine JavaScript keywords with dynamically discovered globals
        let cache = self.cache.read().unwrap();
        let mut completions: Vec<Pair> = JS_KEYWORDS
            .iter()
            .map(|s| s.to_string())
            .chain(cache.globals.iter().cloned())
            .filter(|k| k.starts_with(word))
            .map(|k| Pair {
                display: k.clone(),
                replacement: k,
            })
            .collect();

        // Remove duplicates and sort
        completions.sort_by(|a, b| a.display.cmp(&b.display));
        completions.dedup_by(|a, b| a.display == b.display);

        Ok((start, completions))
    }
}

impl Hinter for JsCompleter {
    type Hint = String;
}

impl Highlighter for JsCompleter {}

impl Validator for JsCompleter {
    fn validate(&self, _ctx: &mut ValidationContext) -> rustyline::Result<ValidationResult> {
        Ok(ValidationResult::Valid(None))
    }
}

impl Helper for JsCompleter {}

fn is_js_identifier_start(c: char) -> bool {
    c == '_' || c == '$' || c.is_ascii_alphabetic()
}

fn is_js_identifier_continue(c: char) -> bool {
    is_js_identifier_start(c) || c.is_ascii_digit()
}

fn is_completion_identifier_char(c: char) -> bool {
    is_js_identifier_continue(c) || c == '.'
}

fn skip_whitespace(source: &str, mut index: usize) -> usize {
    while let Some(ch) = source[index..].chars().next() {
        if !ch.is_whitespace() {
            break;
        }
        index += ch.len_utf8();
    }
    index
}

fn skip_string_or_comment(source: &str, index: usize) -> Option<usize> {
    let rest = &source[index..];

    if rest.starts_with("//") {
        return Some(source.len());
    }

    if rest.starts_with("/*") {
        return Some(
            rest.find("*/")
                .map(|offset| index + offset + 2)
                .unwrap_or(source.len()),
        );
    }

    let quote = rest.chars().next()?;
    if quote != '\'' && quote != '"' && quote != '`' {
        return None;
    }

    let mut escaped = false;
    let mut current = index + quote.len_utf8();
    while let Some(ch) = source[current..].chars().next() {
        current += ch.len_utf8();
        if escaped {
            escaped = false;
            continue;
        }

        if ch == '\\' {
            escaped = true;
            continue;
        }

        if ch == quote {
            break;
        }
    }

    Some(current)
}

fn starts_with_keyword(source: &str, index: usize, keyword: &str) -> bool {
    let rest = &source[index..];
    if !rest.starts_with(keyword) {
        return false;
    }

    let end = index + keyword.len();
    match source[end..].chars().next() {
        Some(next) => !is_js_identifier_continue(next),
        None => true,
    }
}

fn parse_identifier(source: &str, index: usize) -> Option<(String, usize)> {
    let mut chars = source[index..].char_indices();
    let (_, first) = chars.next()?;
    if !is_js_identifier_start(first) {
        return None;
    }

    let mut end = index + first.len_utf8();
    for (offset, ch) in chars {
        if !is_js_identifier_continue(ch) {
            break;
        }
        end = index + offset + ch.len_utf8();
    }

    Some((source[index..end].to_string(), end))
}

fn scan_to_top_level_delimiter(source: &str, mut index: usize) -> (usize, Option<char>) {
    let mut paren_depth: usize = 0;
    let mut bracket_depth: usize = 0;
    let mut brace_depth: usize = 0;

    while index < source.len() {
        if let Some(next_index) = skip_string_or_comment(source, index) {
            index = next_index;
            continue;
        }

        let ch = source[index..].chars().next().unwrap();
        match ch {
            '(' => paren_depth += 1,
            ')' => paren_depth = paren_depth.saturating_sub(1),
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            '{' => brace_depth += 1,
            '}' => {
                if brace_depth == 0 && paren_depth == 0 && bracket_depth == 0 {
                    return (index, Some('}'));
                }
                brace_depth = brace_depth.saturating_sub(1);
            }
            ',' | ';' if paren_depth == 0 && bracket_depth == 0 && brace_depth == 0 => {
                return (index, Some(ch));
            }
            _ => {}
        }

        index += ch.len_utf8();
    }

    (index, None)
}

fn parse_simple_variable_declarations(
    source: &str,
    mut index: usize,
    bindings: &mut Vec<String>,
) -> usize {
    loop {
        index = skip_whitespace(source, index);
        if index >= source.len() {
            return index;
        }

        if let Some((name, next_index)) = parse_identifier(source, index) {
            bindings.push(name);
            index = next_index;
        }

        let (delimiter_index, delimiter) = scan_to_top_level_delimiter(source, index);
        match delimiter {
            Some(',') => index = delimiter_index + 1,
            Some(';') => return delimiter_index + 1,
            Some('}') => return delimiter_index,
            _ => return delimiter_index,
        }
    }
}

fn parse_function_declaration(source: &str, mut index: usize) -> Option<(String, usize)> {
    index = skip_whitespace(source, index);
    if source[index..].starts_with('*') {
        index += '*'.len_utf8();
        index = skip_whitespace(source, index);
    }

    parse_identifier(source, index)
}

fn parse_class_declaration(source: &str, index: usize) -> Option<(String, usize)> {
    parse_identifier(source, skip_whitespace(source, index))
}

pub(crate) fn extract_repl_binding_names(line: &str) -> Vec<String> {
    let mut bindings = Vec::new();
    let mut index = 0;
    let mut paren_depth: usize = 0;
    let mut bracket_depth: usize = 0;
    let mut brace_depth: usize = 0;
    let mut at_statement_start = true;

    while index < line.len() {
        index = skip_whitespace(line, index);
        if index >= line.len() {
            break;
        }

        if let Some(next_index) = skip_string_or_comment(line, index) {
            index = next_index;
            continue;
        }

        if paren_depth == 0 && bracket_depth == 0 && brace_depth == 0 && at_statement_start {
            if starts_with_keyword(line, index, "const") {
                index =
                    parse_simple_variable_declarations(line, index + "const".len(), &mut bindings);
                at_statement_start = line[..index].ends_with(';');
                continue;
            }

            if starts_with_keyword(line, index, "let") {
                index =
                    parse_simple_variable_declarations(line, index + "let".len(), &mut bindings);
                at_statement_start = line[..index].ends_with(';');
                continue;
            }

            if starts_with_keyword(line, index, "var") {
                index =
                    parse_simple_variable_declarations(line, index + "var".len(), &mut bindings);
                at_statement_start = line[..index].ends_with(';');
                continue;
            }

            if starts_with_keyword(line, index, "async") {
                let function_index = skip_whitespace(line, index + "async".len());
                if starts_with_keyword(line, function_index, "function")
                    && let Some((name, next_index)) =
                        parse_function_declaration(line, function_index + "function".len())
                {
                    bindings.push(name);
                    index = next_index;
                    at_statement_start = false;
                    continue;
                }
            }

            if starts_with_keyword(line, index, "function")
                && let Some((name, next_index)) =
                    parse_function_declaration(line, index + "function".len())
            {
                bindings.push(name);
                index = next_index;
                at_statement_start = false;
                continue;
            }

            if starts_with_keyword(line, index, "class")
                && let Some((name, next_index)) =
                    parse_class_declaration(line, index + "class".len())
            {
                bindings.push(name);
                index = next_index;
                at_statement_start = false;
                continue;
            }
        }

        let ch = line[index..].chars().next().unwrap();
        match ch {
            ';' if paren_depth == 0 && bracket_depth == 0 && brace_depth == 0 => {
                at_statement_start = true;
                index += ch.len_utf8();
                continue;
            }
            '(' => paren_depth += 1,
            ')' => paren_depth = paren_depth.saturating_sub(1),
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            '{' => brace_depth += 1,
            '}' => brace_depth = brace_depth.saturating_sub(1),
            _ => {}
        }

        if !ch.is_whitespace() {
            at_statement_start = false;
        }

        index += ch.len_utf8();
    }

    bindings.sort();
    bindings.dedup();
    bindings
}

// Helper function to refresh the completion cache
pub(crate) fn refresh_cache(
    jstime: &mut jstime::JSTime,
    cache: &Arc<RwLock<CompletionCache>>,
    repl_bindings: &FxHashSet<String>,
) {
    let mut globals = jstime.get_global_names();
    globals.extend(repl_bindings.iter().cloned());
    globals.sort();
    globals.dedup();

    // Get properties for each global that looks like an object
    let mut properties = FxHashMap::default();
    for name in &globals {
        // Skip keywords and primitives
        if JS_KEYWORDS.contains(&name.as_str()) || JS_PRIMITIVES.contains(&name.as_str()) {
            continue;
        }

        let props = jstime.get_property_names(name);
        if !props.is_empty() {
            properties.insert(name.clone(), props);
        }
    }

    // Also get nested properties for crypto.subtle
    let subtle_props = jstime.get_property_names("crypto.subtle");
    if !subtle_props.is_empty() {
        properties.insert("crypto.subtle".to_string(), subtle_props);
    }

    // Update the cache
    let mut cache_guard = cache.write().unwrap();
    cache_guard.globals = globals;
    cache_guard.properties = properties;
}
