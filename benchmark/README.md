# LM Benchmark Suite

A benchmark suite for testing LLM code generation quality across LM, TypeScript, and Python. The core hypothesis: LM's strict type system, effect tracking, and functional-first design help LLMs write correct code on the first try.

## Quick Start

```bash
# Run all benchmarks
./benchmark/run_benchmark.sh

# Run a single task manually
lmc run benchmark/tasks/07_fibonacci/test.lm
```

## What This Measures

Each task provides a `prompt.md` that describes a programming problem. An LLM is given this prompt and asked to implement the solution in LM, TypeScript, and Python. We then check:

1. **Correctness** -- Does the generated code produce the expected output?
2. **First-try success rate** -- Does it work without any corrections?
3. **Error category** -- When it fails, what kind of mistake was it?

The hypothesis is that LM's constraints (no mutation, no loops, exhaustive pattern matching, effect tracking) reduce the space of possible mistakes an LLM can make.

## Structure

```
benchmark/
  run_benchmark.sh       # Run all tasks, compare output, report results
  README.md              # This file
  tasks/
    01_hello/
      prompt.md          # Task description (given to the LLM)
      solution.lm        # Reference solution
      test.lm            # Test program (runs solution, prints results)
      expected.txt        # Expected stdout output
    02_reverse_string/
      ...
    ...30 tasks total...
```

## The 30 Tasks

### String Processing (01-06)
| # | Task | Status | Description |
|---|------|--------|-------------|
| 01 | hello | PASS | String greeting function |
| 02 | reverse_string | SKIP | Reverse a string (needs string indexing) |
| 03 | palindrome | SKIP | Palindrome check (needs string indexing) |
| 04 | count_vowels | SKIP | Count vowels (needs char operations) |
| 05 | caesar_cipher | SKIP | Caesar cipher (needs char operations) |
| 06 | string_repeat | PASS | Repeat string N times with separator |

### Number/Math (07-12)
| # | Task | Status | Description |
|---|------|--------|-------------|
| 07 | fibonacci | PASS | Nth Fibonacci number (recursive) |
| 08 | factorial | PASS | Factorial with Result for error handling |
| 09 | gcd | PASS | Greatest common divisor (Euclidean) |
| 10 | is_prime | PASS | Primality test with trial division |
| 11 | collatz | PASS | Collatz sequence step count |
| 12 | power | PASS | Integer exponentiation |

### List Processing (13-18)
| # | Task | Status | Description |
|---|------|--------|-------------|
| 13 | list_sum | SKIP | Sum integers in a list (needs list builtins in type checker) |
| 14 | list_filter | SKIP | Filter list by predicate (needs list builtins) |
| 15 | list_reverse | SKIP | Reverse a list (needs list builtins) |
| 16 | list_contains | SKIP | Check if list contains element (needs list builtins) |
| 17 | list_zip | SKIP | Zip two lists into pairs (needs list builtins) |
| 18 | list_flatten | SKIP | Flatten list of lists (needs list builtins) |

### ADT & Pattern Matching (19-24)
| # | Task | Status | Description |
|---|------|--------|-------------|
| 19 | shape_area | PASS | Area of Circle/Rect/Triangle ADT |
| 20 | option_unwrap | PASS | unwrap_or for Option type |
| 21 | result_chain | PASS | Chain Result-returning operations |
| 22 | expr_eval | PASS | Evaluate arithmetic expression ADT |
| 23 | tree_depth | SKIP | Binary tree depth (needs recursive types) |
| 24 | list_max | PASS | Maximum of values using Option |

### Effect System (25-27)
| # | Task | Status | Description |
|---|------|--------|-------------|
| 25 | pure_computation | PASS | Pure function with IO wrapper |
| 26 | io_greeting | PASS | Multi-line formatted IO output |
| 27 | effect_separation | PASS | FizzBuzz: pure logic + IO layer |

### Error Handling (28-30)
| # | Task | Status | Description |
|---|------|--------|-------------|
| 28 | safe_divide | PASS | Division with Result, chained operations |
| 29 | parse_and_compute | PASS | Parse-then-compute pipeline |
| 30 | validate_input | PASS | Multi-field validation with error propagation |

## Current Results

- **30/30 total** tasks defined
- **19/19 runnable** tasks pass
- **11 skipped** tasks need language features not yet in M4:
  - 4 need string indexing / char operations
  - 6 need list builtins registered in the type checker
  - 1 needs recursive type definitions

## Methodology for LLM Comparison

### Protocol (future work)

1. For each task, give the LLM the `prompt.md` content
2. Ask it to implement the solution in:
   - **LM** -- using LM syntax and constraints
   - **TypeScript** -- idiomatic TypeScript
   - **Python** -- idiomatic Python
3. Run each solution against the test harness
4. Record: pass/fail, number of attempts needed, error category

### What We Expect to Show

LM should have advantages in:
- **Type errors caught at compile time** -- LM's type system prevents many bugs before runtime
- **Effect tracking** -- IO functions must be explicitly marked, preventing accidental side effects
- **Exhaustive matching** -- Pattern matches must cover all cases, preventing unhandled variants
- **No mutation bugs** -- No variable reassignment means no mutation-related bugs
- **No off-by-one in loops** -- No loops means no loop-boundary errors

The tradeoff is that LM code must use recursion instead of loops, which can be harder for LLMs to get right for complex iteration. The benchmark measures whether the safety benefits outweigh the recursion complexity.

## LM Language Notes for Benchmark Authors

Key constraints when writing LM solutions:
- No mutation: use `let` bindings and recursion
- No loops: use recursion for iteration
- `+` for numbers, `++` for strings (no implicit conversion)
- `print` takes a String argument; use `to_string()` to convert
- Pattern matching must be exhaustive
- IO functions must be marked `io fn`
- If/else is an expression and both branches are required
- Function body is a block `{ }` where the last expression is the return value
- `let x = expr; body` is a let-expression where `body` uses `x`
- Modulo: use `a - (a / b) * b` since there is no `%` operator
