#!/bin/bash
# Cross-runtime compliance and performance test runner
# Compares jstime with other JavaScript runtimes (Node.js, Deno, Bun)

# Check if running under bash
if [ -z "$BASH_VERSION" ]; then
    echo "Error: This script requires bash to run."
    echo "Please run with: bash $0"
    exit 1
fi

# Note: This script is compatible with bash 3.2+

# Don't use set -e so that test failures don't stop the script
set +e

# Constants
ERROR_MARKER="ERROR"
ERROR_DETAILS="ERROR||"

# Parse command line arguments
VERBOSE=false

# Show usage information
show_help() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --verbose, -v    Show detailed breakdown for each performance test"
    echo "  --help, -h       Show this help message"
    echo ""
    echo "The test suite will automatically detect available runtimes"
    echo "(jstime, node, deno, bun) and run compliance and performance tests."
}

for arg in "$@"; do
    case "$arg" in
        --verbose|-v)
            VERBOSE=true
            ;;
        --help|-h)
            show_help
            exit 0
            ;;
        *)
            echo "Unknown option: $arg"
            echo "Use --help to see available options"
            exit 1
            ;;
    esac
done

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Build jstime if needed
if [ ! -f "$PROJECT_ROOT/target/release/jstime" ]; then
    echo -e "${YELLOW}Building jstime...${NC}"
    cd "$PROJECT_ROOT"
    cargo build --release
fi

JSTIME="$PROJECT_ROOT/target/release/jstime"

# Function to get version and path for a runtime
get_runtime_info() {
    local runtime=$1
    local version=""
    local path=""
    
    case "$runtime" in
        jstime)
            path="$JSTIME"
            version=$("$JSTIME" --version 2>/dev/null | head -1 || echo "unknown")
            ;;
        node)
            path=$(command -v node)
            version=$(node --version 2>/dev/null || echo "unknown")
            ;;
        deno)
            path=$(command -v deno)
            version=$(deno --version 2>/dev/null | head -1 | awk '{print $2}' || echo "unknown")
            ;;
        bun)
            path=$(command -v bun)
            version=$(bun --version 2>/dev/null || echo "unknown")
            ;;
    esac
    
    echo "$version|$path"
}

# Helper function to store a key-value pair in delimiter-separated strings
# Note: Keys and values are internally generated (runtime names, test names, timing values)
# not from user input, so eval usage is safe in this context.
set_result() {
    local var_name=$1
    local key=$2
    local value=$3
    # The newline is intentional - it separates entries in the string
    eval "$var_name=\"\${$var_name}\${key}|${value}
\""
}

# Helper function to get a value by key from delimiter-separated strings
get_result() {
    local var_name=$1
    local key=$2
    # Safely extract value by matching the key pattern
    eval "echo \"\$$var_name\"" | grep "^${key}|" | cut -d'|' -f2-
}

# Detect available runtimes
RUNTIMES=()
# Use delimiter-separated strings for bash 3.2 compatibility (no associative arrays)
RUNTIME_VERSIONS=""
RUNTIME_PATHS=""

if [ -x "$JSTIME" ]; then
    RUNTIMES+=("jstime")
    info=$(get_runtime_info "jstime")
    set_result RUNTIME_VERSIONS "jstime" "$(echo "$info" | cut -d'|' -f1)"
    set_result RUNTIME_PATHS "jstime" "$(echo "$info" | cut -d'|' -f2)"
fi

if command -v node &> /dev/null; then
    RUNTIMES+=("node")
    info=$(get_runtime_info "node")
    set_result RUNTIME_VERSIONS "node" "$(echo "$info" | cut -d'|' -f1)"
    set_result RUNTIME_PATHS "node" "$(echo "$info" | cut -d'|' -f2)"
fi

if command -v deno &> /dev/null; then
    RUNTIMES+=("deno")
    info=$(get_runtime_info "deno")
    set_result RUNTIME_VERSIONS "deno" "$(echo "$info" | cut -d'|' -f1)"
    set_result RUNTIME_PATHS "deno" "$(echo "$info" | cut -d'|' -f2)"
fi

if command -v bun &> /dev/null; then
    RUNTIMES+=("bun")
    info=$(get_runtime_info "bun")
    set_result RUNTIME_VERSIONS "bun" "$(echo "$info" | cut -d'|' -f1)"
    set_result RUNTIME_PATHS "bun" "$(echo "$info" | cut -d'|' -f2)"
fi

if [ ${#RUNTIMES[@]} -eq 0 ]; then
    echo -e "${RED}Error: No JavaScript runtimes found${NC}"
    exit 1
fi

echo -e "${BLUE}=== Cross-Runtime Test Suite ===${NC}"
echo -e "${BLUE}Available runtimes: ${RUNTIMES[*]}${NC}"
echo ""

# Display runtime information
for runtime in "${RUNTIMES[@]}"; do
    echo -e "${YELLOW}$runtime${NC}"
    echo -e "  Path:    $(get_result RUNTIME_PATHS "$runtime")"
    echo -e "  Version: $(get_result RUNTIME_VERSIONS "$runtime")"
done
echo ""

# Function to run a test file with a specific runtime
run_test() {
    local runtime=$1
    local test_file=$2
    local test_type=$3  # "compliance" or "performance"
    
    case "$runtime" in
        jstime)
            timeout 30 "$JSTIME" "$test_file" 2>&1
            ;;
        node)
            timeout 30 node "$test_file" 2>&1
            ;;
        deno)
            timeout 30 deno run --allow-net "$test_file" 2>&1
            ;;
        bun)
            timeout 30 bun run "$test_file" 2>&1
            ;;
    esac
}

# Run compliance tests
echo -e "${BLUE}=== Compliance Tests ===${NC}"
echo ""

COMPLIANCE_DIR="$SCRIPT_DIR/compliance"
COMPLIANCE_TESTS=(
    "test-console.js"
    "test-timers.js"
    "test-url.js"
    "test-crypto.js"
    "test-performance.js"
    "test-base64.js"
    "test-json.js"
    "test-text-encoding.js"
    "test-event.js"
    "test-streams.js"
    "test-structured-clone.js"
    "test-microtask.js"
)

# Track results using delimiter-separated strings (bash 3.2 compatible)
# Format: "key1|value1\nkey2|value2\n..."
COMPLIANCE_RESULTS=""


for test_file in "${COMPLIANCE_TESTS[@]}"; do
    test_name=$(basename "$test_file" .js)
    echo -e "${YELLOW}Running $test_name...${NC}"
    
    for runtime in "${RUNTIMES[@]}"; do
        printf "  %-10s: " "$runtime"
        
        output=$(run_test "$runtime" "$COMPLIANCE_DIR/$test_file" "compliance" 2>&1 || echo "RUNTIME_ERROR")
        
        # Extract results from output
        result_line=$(echo "$output" | grep "API:" || echo "")
        
        if echo "$output" | grep -q "RUNTIME_ERROR"; then
            echo -e "${RED}ERROR${NC}"
            set_result COMPLIANCE_RESULTS "$runtime-$test_name" "ERROR"
        elif echo "$output" | grep -qE "FAIL:|SyntaxError|ReferenceError|TypeError"; then
            echo -e "${RED}FAILED${NC}"
            set_result COMPLIANCE_RESULTS "$runtime-$test_name" "FAILED"
            echo "$output" | grep -E "FAIL:|Error:" | head -3 | sed 's/^/    /'
        elif [ -n "$result_line" ]; then
            # Extract passed and failed counts
            passed_count=$(echo "$result_line" | grep -oE "[0-9]+ passed" | grep -oE "[0-9]+" || echo "0")
            failed_count=$(echo "$result_line" | grep -oE "[0-9]+ failed" | grep -oE "[0-9]+" || echo "0")
            
            # Check if any tests failed
            if [ "$failed_count" != "0" ] && [ -n "$failed_count" ]; then
                echo -e "${YELLOW}${passed_count} passed, ${failed_count} failed${NC}"
                set_result COMPLIANCE_RESULTS "$runtime-$test_name" "FAILED"
            else
                echo -e "${GREEN}${passed_count} passed ✓${NC}"
                set_result COMPLIANCE_RESULTS "$runtime-$test_name" "PASSED"
            fi
        else
            echo -e "${RED}NO OUTPUT${NC}"
            set_result COMPLIANCE_RESULTS "$runtime-$test_name" "ERROR"
        fi
    done
    echo ""
done

# Run performance tests
echo -e "${BLUE}=== Performance Tests ===${NC}"
echo ""

PERFORMANCE_DIR="$SCRIPT_DIR/performance"
PERFORMANCE_TESTS=(
    "bench-arithmetic.js"
    "bench-strings.js"
    "bench-arrays.js"
    "bench-objects.js"
    "bench-json.js"
    "bench-base64.js"
    "bench-url.js"
    "bench-crypto.js"
    "bench-text-encoding.js"
    "bench-structured-clone.js"
    "bench-event.js"
)

# Store performance results (bash 3.2 compatible)
PERF_RESULTS=""

for test_file in "${PERFORMANCE_TESTS[@]}"; do
    test_name=$(basename "$test_file" .js)
    echo -e "${YELLOW}Running $test_name...${NC}"
    
    for runtime in "${RUNTIMES[@]}"; do
        printf "  %-10s: " "$runtime"
        
        output=$(run_test "$runtime" "$PERFORMANCE_DIR/$test_file" "performance" 2>&1 || echo "$ERROR_MARKER")
        
        if echo "$output" | grep -q '"test"'; then
            # Parse JSON output
            # Extract only the top-level values, not from sub_tests
            elapsed=$(echo "$output" | sed 's/"sub_tests":\[.*\]//' | grep -o '"elapsed_ms":"[^"]*"' | head -1 | cut -d'"' -f4)
            ops_per_ms=$(echo "$output" | sed 's/"sub_tests":\[.*\]//' | grep -o '"ops_per_ms":"[^"]*"' | head -1 | cut -d'"' -f4)
            iterations=$(echo "$output" | sed 's/"sub_tests":\[.*\]//' | grep -o '"iterations":[0-9]*' | head -1 | cut -d':' -f2)
            
            # Validate parsed values
            if [ -z "$elapsed" ] || [ -z "$ops_per_ms" ] || [ -z "$iterations" ]; then
                echo -e "${RED}${ERROR_MARKER} (invalid JSON)${NC}"
                set_result PERF_RESULTS "$runtime-$test_name" "$ERROR_MARKER"
                set_result PERF_DETAILS "$runtime-$test_name" "$ERROR_DETAILS"
            elif [ "$VERBOSE" = true ]; then
                echo -e "${GREEN}${elapsed}ms (total)${NC}"
                
                # Check if sub_tests exist and parse them
                if echo "$output" | grep -q '"sub_tests"'; then
                    # Extract sub_tests array and parse each test
                    # Use a temp file to avoid subshell issues with while-read
                    echo "$output" | grep -o '{"name":"[^"]*","elapsed_ms":"[^"]*","ops_per_ms":"[^"]*"}' > /tmp/subtests_$$.txt 2>&1
                    while IFS= read -r sub_test; do
                        if [ -n "$sub_test" ]; then
                            name=$(echo "$sub_test" | grep -o '"name":"[^"]*"' | cut -d'"' -f4)
                            sub_elapsed=$(echo "$sub_test" | grep -o '"elapsed_ms":"[^"]*"' | cut -d'"' -f4)
                            sub_ops=$(echo "$sub_test" | grep -o '"ops_per_ms":"[^"]*"' | cut -d'"' -f4)
                            printf "      ${GREEN}%-20s: %8sms (%10s ops/ms)${NC}\n" "$name" "$sub_elapsed" "$sub_ops"
                        fi
                    done < /tmp/subtests_$$.txt
                    rm -f /tmp/subtests_$$.txt 2>&1
                fi
                set_result PERF_RESULTS "$runtime-$test_name" "$elapsed"
                # Store full JSON output to temp file for summary
                echo "$output" > "/tmp/perf_details_${runtime}_${test_name}.json"
            else
                echo -e "${GREEN}${elapsed}ms (${ops_per_ms} ops/ms)${NC}"
                set_result PERF_RESULTS "$runtime-$test_name" "$elapsed"
                # Store full JSON output to temp file for summary
                echo "$output" > "/tmp/perf_details_${runtime}_${test_name}.json"
            fi
        else
            echo -e "${RED}${ERROR_MARKER}${NC}"
            set_result PERF_RESULTS "$runtime-$test_name" "$ERROR_MARKER"
            set_result PERF_DETAILS "$runtime-$test_name" "$ERROR_DETAILS"
        fi
    done
    echo ""
done

# Generate summary
echo -e "${BLUE}=== Summary ===${NC}"
echo ""

echo -e "${YELLOW}Compliance Test Results:${NC}"
for runtime in "${RUNTIMES[@]}"; do
    passed=0
    failed=0
    
    for test_file in "${COMPLIANCE_TESTS[@]}"; do
        test_name=$(basename "$test_file" .js)
        result=$(get_result COMPLIANCE_RESULTS "$runtime-$test_name")
        if [ "$result" == "PASSED" ]; then
            ((passed++))
        else
            ((failed++))
        fi
    done
    
    total=$((passed + failed))
    printf "  %-10s: %d/%d passed" "$runtime" "$passed" "$total"
    if [ $failed -eq 0 ]; then
        echo -e " ${GREEN}✓${NC}"
    else
        echo -e " ${RED}($failed failed)${NC}"
    fi
done

echo ""
echo -e "${YELLOW}Performance Comparison:${NC}"
if [ "$VERBOSE" = true ]; then
    echo "  (Lower time is better - showing detailed breakdown)"
else
    echo "  (Lower is better - showing elapsed time in milliseconds)"
fi
echo ""

# Print performance comparison table
for test_file in "${PERFORMANCE_TESTS[@]}"; do
    test_name=$(basename "$test_file" .js | sed 's/bench-//')
    printf "  %-20s" "$test_name:"
    
    # Find the best (lowest) time and worst (highest) time
    best_time=999999
    worst_time=0
    for runtime in "${RUNTIMES[@]}"; do
        time=$(get_result PERF_RESULTS "$runtime-bench-$test_name")
        if [ "$time" != "$ERROR_MARKER" ] && [ -n "$time" ]; then
            if (( $(echo "$time < $best_time" | bc -l 2>/dev/null || echo 0) )); then
                best_time="$time"
            fi
            if (( $(echo "$time > $worst_time" | bc -l 2>/dev/null || echo 0) )); then
                worst_time="$time"
            fi
        fi
    done
    
    for runtime in "${RUNTIMES[@]}"; do
        time=$(get_result PERF_RESULTS "$runtime-bench-$test_name")
        if [ "$time" != "$ERROR_MARKER" ] && [ -n "$time" ]; then
            # Mark the fastest runtime in green, slowest in red, others in yellow
            if [ "$time" == "$best_time" ]; then
                printf " ${GREEN}%-10s${NC}" "$runtime:${time}ms★"
            elif [ "$time" == "$worst_time" ]; then
                printf " ${RED}%-10s${NC}" "$runtime:${time}ms"
            else
                printf " ${YELLOW}%-10s${NC}" "$runtime:${time}ms"
            fi
        fi
    done
    echo ""
    
    # Show detailed breakdown in verbose mode
    if [ "$VERBOSE" = true ]; then
        for runtime in "${RUNTIMES[@]}"; do
            detail_file="/tmp/perf_details_${runtime}_bench-${test_name}.json"
            if [ -f "$detail_file" ]; then
                details=$(cat "$detail_file")
                # Check if this is JSON output with sub_tests
                if echo "$details" | grep -q '"sub_tests"'; then
                    printf "    ${GREEN}%-10s${NC}\n" "$runtime:"
                    # Extract and parse sub_tests using temp file
                    echo "$details" | grep -o '{"name":"[^"]*","elapsed_ms":"[^"]*","ops_per_ms":"[^"]*"}' > /tmp/summary_subtests_$$.txt 2>&1
                    while IFS= read -r sub_test; do
                        if [ -n "$sub_test" ]; then
                            name=$(echo "$sub_test" | grep -o '"name":"[^"]*"' | cut -d'"' -f4)
                            sub_elapsed=$(echo "$sub_test" | grep -o '"elapsed_ms":"[^"]*"' | cut -d'"' -f4)
                            sub_ops=$(echo "$sub_test" | grep -o '"ops_per_ms":"[^"]*"' | cut -d'"' -f4)
                            printf "        %-20s: %8sms (%10s ops/ms)\n" "$name" "$sub_elapsed" "$sub_ops"
                        fi
                    done < /tmp/summary_subtests_$$.txt
                    rm -f /tmp/summary_subtests_$$.txt 2>&1
                fi
            fi
        done
        echo ""
    fi
done

echo ""
echo -e "${BLUE}=== Test Complete ===${NC}"

# Clean up temporary files
rm -f /tmp/perf_details_*.json /tmp/subtests_*.txt /tmp/summary_subtests_*.txt 2>/dev/null
