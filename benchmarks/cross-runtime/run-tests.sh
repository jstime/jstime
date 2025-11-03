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
SELECTED_APIS=""

# Show usage information
show_help() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --api <apis>     Run tests only for specific APIs (comma-separated)"
    echo "                   Use 'all' to run all tests (default)"
    echo "                   Available APIs:"
    echo "                     console, timers, url, crypto, performance, base64,"
    echo "                     json, text-encoding, event, streams, structured-clone,"
    echo "                     microtask, fetch, webassembly, fs, process,"
    echo "                     arithmetic, strings, arrays, objects"
    echo "                   Example: --api crypto,url,json"
    echo "  --verbose, -v    Show detailed breakdown for each performance test"
    echo "  --help, -h       Show this help message"
    echo ""
    echo "The test suite will automatically detect available runtimes"
    echo "(jstime, node, deno, bun) and run compliance and performance tests."
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        --api)
            if [ -z "$2" ] || [[ "$2" == --* ]]; then
                echo "Error: --api requires a value"
                exit 1
            fi
            SELECTED_APIS="$2"
            shift 2
            ;;
        --verbose|-v)
            VERBOSE=true
            shift
            ;;
        --help|-h)
            show_help
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help to see available options"
            exit 1
            ;;
    esac
done

# Set default to all APIs if not specified
if [ -z "$SELECTED_APIS" ]; then
    SELECTED_APIS="all"
fi

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

# Function to check if an API should be tested
should_test_api() {
    local api_name=$1
    
    # If "all" is specified, test everything
    if [ "$SELECTED_APIS" == "all" ]; then
        return 0
    fi
    
    # Check if the API is in the selected list
    if echo "$SELECTED_APIS" | grep -qE "(^|,)${api_name}(,|$)"; then
        return 0
    else
        return 1
    fi
}

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
    "test-fetch.js"
    "test-webassembly.js"
    "test-fs.mjs"
    "test-process.js"
)

# Track results using delimiter-separated strings (bash 3.2 compatible)
# Format: "key1|value1\nkey2|value2\n..."
COMPLIANCE_RESULTS=""


for test_file in "${COMPLIANCE_TESTS[@]}"; do
    test_name=$(basename "$test_file" | sed 's/^test-//' | sed 's/\..*//')
    
    # Check if this API should be tested
    if ! should_test_api "$test_name"; then
        continue
    fi
    
    echo -e "${YELLOW}Running test-$test_name...${NC}"
    
    for runtime in "${RUNTIMES[@]}"; do
        printf "  %-10s: " "$runtime"
        
        output=$(run_test "$runtime" "$COMPLIANCE_DIR/$test_file" "compliance" 2>&1 || echo "RUNTIME_ERROR")
        
        # Extract results from output
        result_line=$(echo "$output" | grep "API:" || echo "")
        
        # Check if API is not available (graceful skip)
        if echo "$output" | grep -q "API not available"; then
            echo -e "${BLUE}SKIPPED (API not available)${NC}"
            set_result COMPLIANCE_RESULTS "$runtime-$test_name" "SKIPPED"
        elif echo "$output" | grep -q "RUNTIME_ERROR"; then
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
    "bench-console.js"
    "bench-performance.js"
    "bench-timers.js"
    "bench-streams.js"
    "bench-fetch.js"
    "bench-webassembly.js"
    "bench-fs.mjs"
    "bench-process.js"
)

# Store performance results (bash 3.2 compatible)
PERF_RESULTS=""

for test_file in "${PERFORMANCE_TESTS[@]}"; do
    test_name=$(basename "$test_file" | sed 's/^bench-//' | sed 's/\..*//')
    
    # Check if this API should be tested
    if ! should_test_api "$test_name"; then
        continue
    fi
    
    echo -e "${YELLOW}Running bench-$test_name...${NC}"
    
    # First pass: collect all results
    for runtime in "${RUNTIMES[@]}"; do
        output=$(run_test "$runtime" "$PERFORMANCE_DIR/$test_file" "performance" 2>&1 || echo "$ERROR_MARKER")
        
        # Check if API is not available (graceful skip)
        if echo "$output" | grep -q '"error":"API not available"'; then
            set_result PERF_RESULTS "$runtime-$test_name" "SKIPPED"
            echo '{"test":"'$test_name'","error":"API not available"}' > "/tmp/perf_details_${runtime}_${test_name}.json"
        elif echo "$output" | grep -q '"test"'; then
            # Parse JSON output
            # Extract only the top-level values, not from sub_tests
            elapsed=$(echo "$output" | sed 's/"sub_tests":\[.*\]//' | grep -o '"elapsed_ms":"[^"]*"' | head -1 | cut -d'"' -f4)
            ops_per_ms=$(echo "$output" | sed 's/"sub_tests":\[.*\]//' | grep -o '"ops_per_ms":"[^"]*"' | head -1 | cut -d'"' -f4)
            iterations=$(echo "$output" | sed 's/"sub_tests":\[.*\]//' | grep -o '"iterations":[0-9]*' | head -1 | cut -d':' -f2)
            
            # Validate parsed values
            if [ -z "$elapsed" ] || [ -z "$ops_per_ms" ] || [ -z "$iterations" ]; then
                set_result PERF_RESULTS "$runtime-$test_name" "$ERROR_MARKER"
                echo "$ERROR_DETAILS" > "/tmp/perf_details_${runtime}_${test_name}.json"
            else
                set_result PERF_RESULTS "$runtime-$test_name" "$elapsed"
                # Store full JSON output to temp file for summary
                echo "$output" > "/tmp/perf_details_${runtime}_${test_name}.json"
            fi
        else
            set_result PERF_RESULTS "$runtime-$test_name" "$ERROR_MARKER"
            echo "$ERROR_DETAILS" > "/tmp/perf_details_${runtime}_${test_name}.json"
        fi
    done
    
    # Second pass: display results with comparisons in verbose mode
    for runtime in "${RUNTIMES[@]}"; do
        printf "  %-10s: " "$runtime"
        
        detail_file="/tmp/perf_details_${runtime}_${test_name}.json"
        if [ -f "$detail_file" ]; then
            output=$(cat "$detail_file")
            
            # Check if API is not available (graceful skip)
            if echo "$output" | grep -q '"error":"API not available"'; then
                echo -e "${BLUE}SKIPPED (API not available)${NC}"
                continue
            elif [ "$output" == "$ERROR_DETAILS" ]; then
                echo -e "${RED}${ERROR_MARKER}${NC}"
                continue
            fi
            
            # Parse JSON output
            elapsed=$(echo "$output" | sed 's/"sub_tests":\[.*\]//' | grep -o '"elapsed_ms":"[^"]*"' | head -1 | cut -d'"' -f4)
            ops_per_ms=$(echo "$output" | sed 's/"sub_tests":\[.*\]//' | grep -o '"ops_per_ms":"[^"]*"' | head -1 | cut -d'"' -f4)
            
            if [ "$VERBOSE" = true ]; then
                echo -e "${GREEN}${elapsed}ms (total)${NC}"
                
                # Check if sub_tests exist and parse them
                if echo "$output" | grep -q '"sub_tests"'; then
                    # Extract sub_tests array and parse each test
                    echo "$output" | grep -o '{"name":"[^"]*","elapsed_ms":"[^"]*","ops_per_ms":"[^"]*"}' > /tmp/subtests_${runtime}_${test_name}.txt 2>&1
                fi
            else
                echo -e "${GREEN}${elapsed}ms (${ops_per_ms} ops/ms)${NC}"
            fi
        else
            echo -e "${RED}${ERROR_MARKER}${NC}"
        fi
    done
    
    # In verbose mode, show sub-test comparisons
    if [ "$VERBOSE" = true ]; then
        # Get list of all sub-test names
        first_runtime="${RUNTIMES[0]}"
        first_file="/tmp/subtests_${first_runtime}_${test_name}.txt"
        
        if [ -f "$first_file" ]; then
            # Process each sub-test
            while IFS= read -r first_subtest; do
                if [ -n "$first_subtest" ]; then
                    subtest_name=$(echo "$first_subtest" | grep -o '"name":"[^"]*"' | cut -d'"' -f4)
                    
                    # Find the fastest time for this sub-test across all runtimes
                    fastest_time=999999
                    for runtime in "${RUNTIMES[@]}"; do
                        subtest_file="/tmp/subtests_${runtime}_${test_name}.txt"
                        if [ -f "$subtest_file" ]; then
                            # Find matching sub-test
                            matching_line=$(grep "\"name\":\"$subtest_name\"" "$subtest_file")
                            if [ -n "$matching_line" ]; then
                                sub_elapsed=$(echo "$matching_line" | grep -o '"elapsed_ms":"[^"]*"' | cut -d'"' -f4)
                                if [ -n "$sub_elapsed" ] && (( $(echo "$sub_elapsed < $fastest_time" | bc -l 2>/dev/null || echo 0) )); then
                                    fastest_time="$sub_elapsed"
                                fi
                            fi
                        fi
                    done
                    
                    # Display results for each runtime with comparison
                    for runtime in "${RUNTIMES[@]}"; do
                        subtest_file="/tmp/subtests_${runtime}_${test_name}.txt"
                        if [ -f "$subtest_file" ]; then
                            matching_line=$(grep "\"name\":\"$subtest_name\"" "$subtest_file")
                            if [ -n "$matching_line" ]; then
                                sub_elapsed=$(echo "$matching_line" | grep -o '"elapsed_ms":"[^"]*"' | cut -d'"' -f4)
                                sub_ops=$(echo "$matching_line" | grep -o '"ops_per_ms":"[^"]*"' | cut -d'"' -f4)
                                
                                # Calculate percentage difference from fastest
                                if [ "$sub_elapsed" == "$fastest_time" ]; then
                                    printf "      ${GREEN}%-10s %-20s: %8sms (%10s ops/ms) ★ fastest${NC}\n" "$runtime" "$subtest_name" "$sub_elapsed" "$sub_ops"
                                else
                                    delta=$(echo "scale=1; ($sub_elapsed - $fastest_time) / $fastest_time * 100" | bc -l 2>/dev/null || echo "0")
                                    # Only show delta if it's >= 0.1%
                                    if (( $(echo "$delta >= 0.1" | bc -l 2>/dev/null || echo 0) )); then
                                        printf "      ${YELLOW}%-10s %-20s: %8sms (%10s ops/ms) +%.1f%%${NC}\n" "$runtime" "$subtest_name" "$sub_elapsed" "$sub_ops" "$delta"
                                    else
                                        printf "      %-10s %-20s: %8sms (%10s ops/ms)\n" "$runtime" "$subtest_name" "$sub_elapsed" "$sub_ops"
                                    fi
                                fi
                            fi
                        fi
                    done
                fi
            done < "$first_file"
        fi
        
        # Clean up temp files
        rm -f /tmp/subtests_*_${test_name}.txt 2>&1
    fi
    
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
        test_name=$(basename "$test_file" .js | sed 's/^test-//')
        
        # Check if this API was tested
        if ! should_test_api "$test_name"; then
            continue
        fi
        
        result=$(get_result COMPLIANCE_RESULTS "$runtime-$test_name")
        if [ "$result" == "PASSED" ]; then
            ((passed++))
        else
            ((failed++))
        fi
    done
    
    # Only show summary if we ran any tests
    if [ $((passed + failed)) -gt 0 ]; then
        total=$((passed + failed))
        printf "  %-10s: %d/%d passed" "$runtime" "$passed" "$total"
        if [ $failed -eq 0 ]; then
            echo -e " ${GREEN}✓${NC}"
        else
            echo -e " ${RED}($failed failed)${NC}"
        fi
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
    
    # Check if this API was tested
    if ! should_test_api "$test_name"; then
        continue
    fi
    
    printf "  %-20s" "$test_name:"
    
    # Find the best (lowest) time and worst (highest) time
    best_time=999999
    worst_time=0
    for runtime in "${RUNTIMES[@]}"; do
        time=$(get_result PERF_RESULTS "$runtime-$test_name")
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
        time=$(get_result PERF_RESULTS "$runtime-$test_name")
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
        # First, collect all sub-test results to find fastest for each sub-test
        # Extract sub-tests to temp files
        for runtime in "${RUNTIMES[@]}"; do
            detail_file="/tmp/perf_details_${runtime}_${test_name}.json"
            if [ -f "$detail_file" ]; then
                details=$(cat "$detail_file")
                if echo "$details" | grep -q '"sub_tests"'; then
                    echo "$details" | grep -o '{"name":"[^"]*","elapsed_ms":"[^"]*","ops_per_ms":"[^"]*"}' > "/tmp/summary_subtests_${runtime}_${test_name}.txt" 2>&1
                fi
            fi
        done
        
        # Get first runtime's sub-tests as the reference list
        first_runtime="${RUNTIMES[0]}"
        first_file="/tmp/summary_subtests_${first_runtime}_${test_name}.txt"
        
        if [ -f "$first_file" ]; then
            # Process each sub-test
            while IFS= read -r first_subtest; do
                if [ -n "$first_subtest" ]; then
                    subtest_name=$(echo "$first_subtest" | grep -o '"name":"[^"]*"' | cut -d'"' -f4)
                    
                    # Find the fastest time for this sub-test across all runtimes
                    fastest_time=999999
                    fastest_runtime=""
                    for runtime in "${RUNTIMES[@]}"; do
                        subtest_file="/tmp/summary_subtests_${runtime}_${test_name}.txt"
                        if [ -f "$subtest_file" ]; then
                            matching_line=$(grep "\"name\":\"$subtest_name\"" "$subtest_file")
                            if [ -n "$matching_line" ]; then
                                sub_elapsed=$(echo "$matching_line" | grep -o '"elapsed_ms":"[^"]*"' | cut -d'"' -f4)
                                if [ -n "$sub_elapsed" ] && (( $(echo "$sub_elapsed < $fastest_time" | bc -l 2>/dev/null || echo 0) )); then
                                    fastest_time="$sub_elapsed"
                                    fastest_runtime="$runtime"
                                fi
                            fi
                        fi
                    done
                    
                    # Display results for each runtime with comparison
                    printf "    ${YELLOW}%-20s${NC}\n" "$subtest_name:"
                    for runtime in "${RUNTIMES[@]}"; do
                        subtest_file="/tmp/summary_subtests_${runtime}_${test_name}.txt"
                        if [ -f "$subtest_file" ]; then
                            matching_line=$(grep "\"name\":\"$subtest_name\"" "$subtest_file")
                            if [ -n "$matching_line" ]; then
                                sub_elapsed=$(echo "$matching_line" | grep -o '"elapsed_ms":"[^"]*"' | cut -d'"' -f4)
                                sub_ops=$(echo "$matching_line" | grep -o '"ops_per_ms":"[^"]*"' | cut -d'"' -f4)
                                
                                # Calculate percentage difference from fastest
                                if [ "$runtime" == "$fastest_runtime" ]; then
                                    printf "        ${GREEN}%-10s: %8sms (%10s ops/ms) ★ fastest${NC}\n" "$runtime" "$sub_elapsed" "$sub_ops"
                                else
                                    delta=$(echo "scale=1; ($sub_elapsed - $fastest_time) / $fastest_time * 100" | bc -l 2>/dev/null || echo "0")
                                    # Only show delta if it's >= 0.1%
                                    if (( $(echo "$delta >= 0.1" | bc -l 2>/dev/null || echo 0) )); then
                                        printf "        %-10s: %8sms (%10s ops/ms) ${YELLOW}+%.1f%%${NC}\n" "$runtime" "$sub_elapsed" "$sub_ops" "$delta"
                                    else
                                        printf "        %-10s: %8sms (%10s ops/ms)\n" "$runtime" "$sub_elapsed" "$sub_ops"
                                    fi
                                fi
                            fi
                        fi
                    done
                fi
            done < "$first_file"
        fi
        
        # Clean up temp files for this test
        rm -f /tmp/summary_subtests_*_${test_name}.txt 2>&1
        echo ""
    fi
done

echo ""
echo -e "${BLUE}=== Test Complete ===${NC}"

# Clean up temporary files
rm -f /tmp/perf_details_*.json /tmp/subtests_*.txt /tmp/summary_subtests_*.txt 2>/dev/null
