# Documentation Index

Welcome to the jstime documentation! This index helps you find the information you need.

## 🚀 Getting Started

Start here if you're new to jstime:

| Document | Description | When to Read |
|----------|-------------|--------------|
| **[README.md](./README.md)** | Project overview and features | First time here |
| **[QUICK_START.md](./QUICK_START.md)** | 5-minute setup guide | Want to start coding now |
| **[examples/README.md](./examples/README.md)** | Runnable code examples | Learning by example |

## 👨‍💻 For Contributors

Essential guides for contributing to jstime:

| Document | Description | When to Read |
|----------|-------------|--------------|
| **[CONTRIBUTING.md](./CONTRIBUTING.md)** | How to contribute | Before your first PR |
| **[CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md)** | Community guidelines | Before participating |
| **[DEVELOPMENT.md](./DEVELOPMENT.md)** | Development workflow | Setting up your environment |
| **[ARCHITECTURE.md](./ARCHITECTURE.md)** | System design and architecture | Understanding the codebase |

## 📚 API Documentation

Documentation for using jstime:

| Document | Description | When to Read |
|----------|-------------|--------------|
| **[docs/FEATURES.md](./docs/FEATURES.md)** | Complete API reference | Looking for specific APIs |
| **[docs/FETCH_API.md](./docs/FETCH_API.md)** | Detailed Fetch API docs | Using HTTP features |
| **[core/README.md](./core/README.md)** | Core library usage | Embedding jstime |

## 🔧 Component Documentation

Detailed docs for specific parts of the codebase:

| Document | Description | When to Read |
|----------|-------------|--------------|
| **[core/src/builtins/README.md](./core/src/builtins/README.md)** | Built-in API implementation | Adding new APIs |
| **[core/tests/README.md](./core/tests/README.md)** | Testing guide | Writing tests |
| **[core/tests/CONFORMANCE_TESTS.md](./core/tests/CONFORMANCE_TESTS.md)** | Standards compliance | Understanding conformance tests |

## 🎯 By Use Case

Find the right documentation for your needs:

### I want to...

#### Use jstime
- 📖 Start with [README.md](./README.md)
- 🔍 Browse [docs/FEATURES.md](./docs/FEATURES.md) for available APIs
- 💡 Check [examples/](./examples/) for code samples

#### Contribute to jstime
- ⚡ Begin with [QUICK_START.md](./QUICK_START.md)
- 📋 Read [CONTRIBUTING.md](./CONTRIBUTING.md)
- 🛠️ Follow [DEVELOPMENT.md](./DEVELOPMENT.md)
- 🏗️ Understand [ARCHITECTURE.md](./ARCHITECTURE.md)

#### Add a new API
- 📚 Read [DEVELOPMENT.md - Adding New Built-in APIs](./DEVELOPMENT.md#adding-new-built-in-apis)
- 🔧 Check [core/src/builtins/README.md](./core/src/builtins/README.md)
- ✅ Study [core/tests/README.md](./core/tests/README.md) for testing

#### Write tests
- 📖 Read [core/tests/README.md](./core/tests/README.md)
- 📋 For conformance tests: [core/tests/CONFORMANCE_TESTS.md](./core/tests/CONFORMANCE_TESTS.md)
- 💡 Look at existing tests for patterns

#### Understand the architecture
- 🏗️ Read [ARCHITECTURE.md](./ARCHITECTURE.md)
- 🔍 Browse [core/src/builtins/README.md](./core/src/builtins/README.md) for API patterns
- 📊 Check [core/README.md](./core/README.md) for library structure

#### Embed jstime in my Rust app
- 📖 Start with [core/README.md](./core/README.md)
- 📚 Reference [docs/FEATURES.md](./docs/FEATURES.md) for available APIs
- 💡 See [ARCHITECTURE.md](./ARCHITECTURE.md) for design understanding

#### Report a bug or request a feature
- 📋 Read [CONTRIBUTING.md](./CONTRIBUTING.md)
- 🐛 Open an issue on [GitHub](https://github.com/jstime/jstime/issues)

#### Learn about governance
- 📖 Read [GOVERNANCE.md](./GOVERNANCE.md)
- 👥 Check [README.md](./README.md) for team members

## 📂 Documentation by Location

```
jstime/
├── README.md                           # Project overview
├── QUICK_START.md                      # 5-minute setup guide
├── CONTRIBUTING.md                     # Contribution guidelines
├── CODE_OF_CONDUCT.md                  # Community standards
├── DEVELOPMENT.md                      # Development workflow
├── ARCHITECTURE.md                     # System architecture
├── GOVERNANCE.md                       # Project governance
├── PERFORMANCE.md                      # Performance notes
├── RELEASING.md                        # Release process
├── DOCUMENTATION_INDEX.md              # This file
│
├── docs/
│   ├── FEATURES.md                     # Complete API reference
│   └── FETCH_API.md                    # Fetch API details
│
├── core/
│   ├── README.md                       # Core library docs
│   ├── src/
│   │   └── builtins/
│   │       └── README.md              # Built-in API guide
│   └── tests/
│       ├── README.md                  # Testing guide
│       └── CONFORMANCE_TESTS.md       # Conformance testing
│
├── examples/
│   └── README.md                      # Example scripts guide
│
└── cli/
    └── README.md                      # CLI documentation (symlink)
```

## 🎓 Learning Path

Recommended reading order for new contributors:

### Level 1: Getting Started (30 minutes)
1. [README.md](./README.md) - Understand what jstime is
2. [QUICK_START.md](./QUICK_START.md) - Get set up and running
3. [examples/README.md](./examples/README.md) - Run some examples

### Level 2: Contributing Basics (1 hour)
4. [CONTRIBUTING.md](./CONTRIBUTING.md) - Learn contribution process
5. [DEVELOPMENT.md](./DEVELOPMENT.md) - Understand the workflow
6. [core/tests/README.md](./core/tests/README.md) - Learn testing patterns

### Level 3: Deep Understanding (2-3 hours)
7. [ARCHITECTURE.md](./ARCHITECTURE.md) - Understand the design
8. [core/src/builtins/README.md](./core/src/builtins/README.md) - Learn API patterns
9. Browse the actual source code with new understanding

### Level 4: Advanced Topics (ongoing)
10. [docs/FEATURES.md](./docs/FEATURES.md) - Complete API knowledge
11. [core/tests/CONFORMANCE_TESTS.md](./core/tests/CONFORMANCE_TESTS.md) - Standards compliance
12. Dig into specific components based on your interests

## 🔍 Finding Information

### By Topic

- **Setup & Installation**: QUICK_START.md, DEVELOPMENT.md
- **Using jstime**: README.md, docs/FEATURES.md, examples/
- **API Reference**: docs/FEATURES.md, docs/FETCH_API.md
- **Contributing**: CONTRIBUTING.md, CODE_OF_CONDUCT.md
- **Development**: DEVELOPMENT.md, ARCHITECTURE.md
- **Testing**: core/tests/README.md, core/tests/CONFORMANCE_TESTS.md
- **Built-ins**: core/src/builtins/README.md
- **Architecture**: ARCHITECTURE.md
- **Governance**: GOVERNANCE.md
- **Performance**: PERFORMANCE.md
- **Releasing**: RELEASING.md

### By Role

**End User / Developer**:
- README.md → docs/FEATURES.md → examples/

**New Contributor**:
- QUICK_START.md → CONTRIBUTING.md → DEVELOPMENT.md

**Experienced Contributor**:
- ARCHITECTURE.md → core/src/builtins/README.md → core/tests/README.md

**Embedder**:
- core/README.md → docs/FEATURES.md → ARCHITECTURE.md

**Maintainer**:
- GOVERNANCE.md → RELEASING.md → All other docs

## 📞 Getting Help

Can't find what you're looking for?

1. **Search the docs**: Use your editor's search or GitHub's search
2. **Check examples**: Look in [examples/](./examples/) for code samples
3. **Ask questions**: Use [GitHub Discussions](https://github.com/jstime/jstime/discussions)
4. **Report issues**: Open an [issue](https://github.com/jstime/jstime/issues) if something is unclear

## 🤝 Improving Documentation

Found a problem or want to improve the docs?

1. Check [CONTRIBUTING.md](./CONTRIBUTING.md)
2. Documentation PRs are always welcome!
3. Even small improvements help (typos, clarity, examples)

## 📊 Documentation Coverage

Our documentation covers:

✅ **Getting Started**: Quick Start, setup instructions, first steps
✅ **API Reference**: Complete API documentation with examples
✅ **Development**: Workflows, patterns, guidelines
✅ **Architecture**: System design, components, data flow
✅ **Testing**: Test patterns, conformance, coverage
✅ **Contributing**: Process, guidelines, code of conduct
✅ **Examples**: Runnable code for all major features

## 🔗 External Resources

- **[V8 Documentation](https://v8.dev/docs)** - JavaScript engine
- **[Rust Documentation](https://doc.rust-lang.org/)** - Rust language
- **[WHATWG Standards](https://spec.whatwg.org/)** - Web standards
- **[GitHub Repository](https://github.com/jstime/jstime)** - Source code

---

**Last Updated**: This index reflects the current state of documentation as of the latest commit.

**Feedback**: If you have suggestions for improving this index or the documentation in general, please open an issue or discussion on GitHub!
