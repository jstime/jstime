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

# Detect available runtimes
RUNTIMES=()

if [ -x "$JSTIME" ]; then
    RUNTIMES+=("jstime")
fi

if command -v node &> /dev/null; then
    RUNTIMES+=("node")
fi

if command -v deno &> /dev/null; then
    RUNTIMES+=("deno")
fi

if command -v bun &> /dev/null; then
    RUNTIMES+=("bun")
fi

if [ ${#RUNTIMES[@]} -eq 0 ]; then
    echo -e "${RED}Error: No JavaScript runtimes found${NC}"
    exit 1
fi

echo -e "${BLUE}=== Cross-Runtime Test Suite ===${NC}"
echo -e "${BLUE}Available runtimes: ${RUNTIMES[*]}${NC}"
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
)

# Track results using delimiter-separated strings (bash 3.2 compatible)
# Format: "key1|value1\nkey2|value2\n..."
COMPLIANCE_RESULTS=""

# Helper function to store a key-value pair
set_result() {
    local var_name=$1
    local key=$2
    local value=$3
    eval "$var_name=\"\${$var_name}\${key}|${value}
\""
}

# Helper function to get a value by key
get_result() {
    local var_name=$1
    local key=$2
    eval "echo \"\$$var_name\"" | grep "^${key}|" | cut -d'|' -f2-
}

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
)

# Store performance results (bash 3.2 compatible)
PERF_RESULTS=""

for test_file in "${PERFORMANCE_TESTS[@]}"; do
    test_name=$(basename "$test_file" .js)
    echo -e "${YELLOW}Running $test_name...${NC}"
    
    for runtime in "${RUNTIMES[@]}"; do
        printf "  %-10s: " "$runtime"
        
        output=$(run_test "$runtime" "$PERFORMANCE_DIR/$test_file" "performance" 2>&1 || echo "ERROR")
        
        if echo "$output" | grep -q '"test"'; then
            # Parse JSON output
            elapsed=$(echo "$output" | grep -o '"elapsed_ms":"[^"]*"' | cut -d'"' -f4)
            ops_per_ms=$(echo "$output" | grep -o '"ops_per_ms":"[^"]*"' | cut -d'"' -f4)
            
            echo -e "${GREEN}${elapsed}ms (${ops_per_ms} ops/ms)${NC}"
            set_result PERF_RESULTS "$runtime-$test_name" "$elapsed"
        else
            echo -e "${RED}ERROR${NC}"
            set_result PERF_RESULTS "$runtime-$test_name" "ERROR"
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
echo "  (Lower is better - showing elapsed time in milliseconds)"
echo ""

# Print performance comparison table
for test_file in "${PERFORMANCE_TESTS[@]}"; do
    test_name=$(basename "$test_file" .js | sed 's/bench-//')
    printf "  %-20s" "$test_name:"
    
    # Find the best (lowest) time
    best_time=999999
    for runtime in "${RUNTIMES[@]}"; do
        time=$(get_result PERF_RESULTS "$runtime-bench-$test_name")
        if [ "$time" != "ERROR" ] && [ -n "$time" ]; then
            if (( $(echo "$time < $best_time" | bc -l 2>/dev/null || echo 0) )); then
                best_time="$time"
            fi
        fi
    done
    
    for runtime in "${RUNTIMES[@]}"; do
        time=$(get_result PERF_RESULTS "$runtime-bench-$test_name")
        if [ "$time" != "ERROR" ] && [ -n "$time" ]; then
            # Mark the fastest runtime
            if [ "$time" == "$best_time" ]; then
                printf " ${GREEN}%-10s${NC}" "$runtime:${time}ms★"
            else
                printf " %-10s" "$runtime:${time}ms"
            fi
        fi
    done
    echo ""
done

echo ""
echo -e "${BLUE}=== Test Complete ===${NC}"
