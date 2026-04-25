#!/bin/bash
# LM Benchmark Suite Runner
#
# Runs all 30 benchmark tasks and reports results.
# Usage: ./benchmark/run_benchmark.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
TASKS_DIR="$SCRIPT_DIR/tasks"

export PATH="$HOME/.cargo/bin:$PATH"

# Build the compiler first
echo "Building lmc..."
cargo build --bin lmc --manifest-path "$PROJECT_DIR/Cargo.toml" 2>/dev/null
LMC="$PROJECT_DIR/target/debug/lmc"
echo ""

passed=0
failed=0
skipped=0
total=0
failed_tasks=""
skipped_tasks=""

# Iterate over task directories in sorted order
for task_dir in "$TASKS_DIR"/*/; do
    task_name=$(basename "$task_dir")
    test_file="$task_dir/test.lm"
    expected_file="$task_dir/expected.txt"
    total=$((total + 1))

    if [ ! -f "$test_file" ] || [ ! -f "$expected_file" ]; then
        printf "  %-30s  MISSING FILES\n" "$task_name"
        failed=$((failed + 1))
        failed_tasks="$failed_tasks $task_name"
        continue
    fi

    # Run the test
    actual=$("$LMC" run "$test_file" 2>/dev/null || echo "__LMC_ERROR__")
    expected=$(cat "$expected_file")

    # Check if this is a SKIP task
    if echo "$actual" | grep -q "^SKIP:"; then
        printf "  %-30s  SKIP\n" "$task_name"
        skipped=$((skipped + 1))
        skipped_tasks="$skipped_tasks $task_name"
        # Still count as passed if the SKIP message matches expected
        if [ "$actual" = "$expected" ]; then
            passed=$((passed + 1))
        else
            failed=$((failed + 1))
            failed_tasks="$failed_tasks $task_name"
        fi
        continue
    fi

    # Check for runtime errors
    if echo "$actual" | grep -q "__LMC_ERROR__"; then
        printf "  %-30s  FAIL (runtime error)\n" "$task_name"
        failed=$((failed + 1))
        failed_tasks="$failed_tasks $task_name"
        continue
    fi

    # Compare output
    if [ "$actual" = "$expected" ]; then
        printf "  %-30s  PASS\n" "$task_name"
        passed=$((passed + 1))
    else
        printf "  %-30s  FAIL\n" "$task_name"
        failed=$((failed + 1))
        failed_tasks="$failed_tasks $task_name"
        # Show diff for debugging
        echo "    Expected:"
        echo "$expected" | head -3 | sed 's/^/      /'
        echo "    Actual:"
        echo "$actual" | head -3 | sed 's/^/      /'
    fi
done

echo ""
echo "================================"
echo "Results: $passed/$total passed"
echo "  Runnable: $((passed - skipped))/$((total - skipped)) passed"
echo "  Skipped:  $skipped (missing language features)"
if [ -n "$failed_tasks" ]; then
    echo "  Failed:  $failed_tasks"
fi
echo "================================"

# Exit with error if any non-skip tests failed
if [ $((failed)) -gt 0 ]; then
    exit 1
fi
