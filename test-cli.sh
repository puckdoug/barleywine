#!/bin/bash

# Barleywine CLI Test Script
# Tests all command-line interface functionality

set -e

echo "🍺 Barleywine CLI Test Suite"
echo "=============================="
echo

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test counters
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0

# Helper functions
run_test() {
    local test_name="$1"
    local command="$2"
    local expected_exit_code="${3:-0}"
    local should_contain="$4"
    local should_not_contain="$5"

    echo -n "Testing: $test_name... "
    TESTS_RUN=$((TESTS_RUN + 1))

    # Run the command and capture output and exit code
    set +e
    output=$(eval "$command" 2>&1)
    actual_exit_code=$?
    set -e

    # Check exit code
    if [ "$actual_exit_code" -ne "$expected_exit_code" ]; then
        echo -e "${RED}FAIL${NC}"
        echo "  Expected exit code: $expected_exit_code"
        echo "  Actual exit code: $actual_exit_code"
        echo "  Output: $output"
        TESTS_FAILED=$((TESTS_FAILED + 1))
        return 1
    fi

    # Check if output should contain certain text
    if [ -n "$should_contain" ] && ! echo "$output" | grep -q "$should_contain"; then
        echo -e "${RED}FAIL${NC}"
        echo "  Output should contain: $should_contain"
        echo "  Actual output: $output"
        TESTS_FAILED=$((TESTS_FAILED + 1))
        return 1
    fi

    # Check if output should NOT contain certain text
    if [ -n "$should_not_contain" ] && echo "$output" | grep -q "$should_not_contain"; then
        echo -e "${RED}FAIL${NC}"
        echo "  Output should NOT contain: $should_not_contain"
        echo "  Actual output: $output"
        TESTS_FAILED=$((TESTS_FAILED + 1))
        return 1
    fi

    echo -e "${GREEN}PASS${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
    return 0
}

# Build the project first
echo "Building project..."
cargo build --quiet
echo

# Test 1: Help flag (long form)
run_test "Help flag (--help)" \
    "cargo run --quiet -- --help" \
    0 \
    "A high-performance web platform"

# Test 2: Help flag (short form)
run_test "Help flag (-h)" \
    "cargo run --quiet -- -h" \
    0 \
    "USAGE:"

# Test 3: Version flag (long form)
run_test "Version flag (--version)" \
    "cargo run --quiet -- --version" \
    0 \
    "barleywine 0.1.0"

# Test 4: Version flag (short form)
run_test "Version flag (-V)" \
    "cargo run --quiet -- -V" \
    0 \
    "0.1.0"

# Test 5: Verify flag
run_test "Verify flag (--verify)" \
    "cargo run --quiet -- --verify" \
    0 \
    "Configuration is valid!"

# Test 6: Valid log levels
for level in error warn info debug trace; do
    run_test "Valid log level: $level" \
        "cargo run --quiet -- --verify --loglevel $level" \
        0 \
        "Log level: $level"
done

# Test 7: Invalid log level
run_test "Invalid log level" \
    "cargo run --quiet -- --loglevel invalid" \
    1 \
    "Invalid log level"

# Test 8: Config file that exists
run_test "Valid config file" \
    "cargo run --quiet -- --config config.toml --verify" \
    0 \
    "Config file: config.toml"

# Test 9: Config file that doesn't exist
run_test "Non-existent config file" \
    "cargo run --quiet -- --config nonexistent.toml --verify" \
    1 \
    "does not exist"

# Test 10: Valid log file (create logs directory if it doesn't exist)
mkdir -p logs
run_test "Valid log file" \
    "cargo run --quiet -- --log logs/test.log --verify" \
    0 \
    "Log file: logs/test.log"

# Test 11: Log file in non-existent directory
run_test "Log file in non-existent directory" \
    "cargo run --quiet -- --log baddir/test.log --verify" \
    1 \
    "does not exist"

# Test 12: Multiple options combined
run_test "Multiple valid options" \
    "cargo run --quiet -- --config config.toml --loglevel debug --log logs/test.log --verify" \
    0 \
    "Configuration is valid!"

# Test 13: Case sensitivity for log levels
run_test "Log level case sensitivity (uppercase)" \
    "cargo run --quiet -- --loglevel INFO --verify" \
    0 \
    "Log level: INFO"

run_test "Log level case sensitivity (mixed case)" \
    "cargo run --quiet -- --loglevel Debug --verify" \
    0 \
    "Log level: Debug"

# Test 14: Short form config option
run_test "Short form config (-c)" \
    "cargo run --quiet -- -c config.toml --verify" \
    0 \
    "Config file: config.toml"

# Test 15: Webroot detection
run_test "Webroot directory detection" \
    "cargo run --quiet -- --verify" \
    0 \
    "Webroot directory: ✅"

# Test 16: Markdown support detection
run_test "Markdown support detection" \
    "cargo run --quiet -- --verify" \
    0 \
    "Markdown support: ✅"

# Test 17: Rocket framework detection
run_test "Rocket framework detection" \
    "cargo run --quiet -- --verify" \
    0 \
    "Rocket framework: ✅"

# Test 18: Default values
run_test "Default log level" \
    "cargo run --quiet -- --verify" \
    0 \
    "Log level: info"

run_test "Default config" \
    "cargo run --quiet -- --verify" \
    0 \
    "Config file: default"

run_test "Default log output" \
    "cargo run --quiet -- --verify" \
    0 \
    "Log file: stdout"

# Test 19: Test without webroot directory (create backup and test)
if [ -d "webroot" ]; then
    echo "Testing without webroot directory..."
    mv webroot webroot.backup
    run_test "Missing webroot directory" \
        "cargo run --quiet -- --verify" \
        0 \
        "Webroot directory: ❌"
    mv webroot.backup webroot
fi

# Test 20: Edge cases - empty strings and special characters
run_test "Empty log level (should fail)" \
    "cargo run --quiet -- --loglevel '' --verify" \
    1 \
    "Invalid log level"

# Summary
echo
echo "=============================="
echo "Test Summary:"
echo -e "  Total tests run: ${BLUE}$TESTS_RUN${NC}"
echo -e "  Tests passed: ${GREEN}$TESTS_PASSED${NC}"
echo -e "  Tests failed: ${RED}$TESTS_FAILED${NC}"

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "\n🎉 ${GREEN}All tests passed!${NC}"
    exit 0
else
    echo -e "\n❌ ${RED}Some tests failed.${NC}"
    exit 1
fi
