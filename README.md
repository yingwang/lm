# LM — A Programming Language Optimized for LLM Code Generation

LM is a programming language designed so that LLMs write correct code on the first try. Every design decision eliminates a category of bugs that LLMs commonly produce: no mutation, no implicit conversions, no null, no exceptions, no inheritance. The compiler speaks both human and JSON, so LLM agents can parse diagnostics and self-correct.

**Goal:** On a standard set of programming tasks, Claude writing LM should achieve significantly higher first-pass correctness than TypeScript or Python.

## Language at a Glance

```lm
// Pure by default — no side effects allowed
fn add(a: Int, b: Int) -> Int {
    a + b
}

// IO must be explicitly marked — compiler enforces infectiousness
io fn greet(name: String) -> Unit {
    print("Hello, " ++ name ++ "!")
}

// Algebraic data types + exhaustive pattern matching
type Shape =
    | Circle(Float)
    | Rect(Float, Float)

fn area(s: Shape) -> Float {
    match s {
        Circle(r) -> 3.14159 * r * r,
        Rect(w, h) -> w * h,
    }
}

// No null, no exceptions — Option and Result only
fn safe_div(a: Int, b: Int) -> Result<Int, String> {
    match b {
        0 -> Err("division by zero"),
        _ -> Ok(a / b),
    }
}

// Immutable bindings — all "modification" returns new values
let x = 10;
let y = add(x, 5);
```

## Core Rules

| Rule | Rationale |
|------|-----------|
| Complete immutability, no `mut` | Eliminates aliasing bugs, race conditions, spooky action at a distance |
| `+` for numbers only, `++` for strings | No ambiguity about what `+` does |
| No implicit type conversion | `Int` + `Float` is a compile error, must use `to_float(x)` |
| No null/nil, use `Option<T>` | Forces handling of absence at every call site |
| No exceptions, use `Result<T, E>` | Error paths are visible in the type signature |
| No inheritance/traits/method dispatch | One way to do things: functions + pattern matching |
| No macros/reflection/metaprogramming | What you see is what runs |
| Effect system: `pure` (default) / `io` | Pure functions can't call IO — compiler enforces it |
| Exhaustive pattern matching | Forget a case? Compile error. |
| Hindley-Milner type inference | Types inferred globally, explicit annotations encouraged |

## Diagnostics

Every compiler diagnostic is available in two formats:

**Human-readable** (rustc-style, with colors):
```
error[E0001]: unrecognized character `@`
--> examples/errors.lm:5:9
  |
5 | let y = @x;
  |         ^ not a valid LM token
  |
  = help: LM uses ASCII operators and identifiers
  = quickfix: remove `@`
```

**JSON** (for LLM agents and tooling):
```json
{
  "code": "E0001",
  "severity": "Error",
  "message": "unrecognized character `@`",
  "span": {"file_id": 0, "start": 105, "end": 106},
  "labels": [{"span": {"file_id": 0, "start": 105, "end": 106}, "message": "not a valid LM token"}],
  "notes": [],
  "help": "LM uses ASCII operators and identifiers",
  "quickfixes": [{"span": {"file_id": 0, "start": 105, "end": 106}, "replacement": "", "description": "remove `@`"}]
}
```

Error code ranges:
- `E0001-E0099` — Lexer errors
- `E0100-E0199` — Parser errors
- `E0200-E0299` — Type checking errors
- `E0300-E0399` — Effect checking errors
- `E0400-E0499` — Pattern matching exhaustiveness
- `E0500-E0599` — Runtime errors

Error codes are stable — once assigned, their meaning never changes.

## CLI (`lmc`)

```sh
lmc tokenize <file>              # Output token stream
lmc parse <file>                 # Output AST (JSON)
lmc check <file>                 # Type check without executing
lmc run <file>                   # Type check + execute

# Global flags
--format=human|json              # Diagnostic output format (default: human)
```

## Project Structure

```
lm/
├── crates/
│   ├── lm-diagnostics/    # Span, Diagnostic, ErrorCode, human + JSON rendering
│   ├── lm-lexer/          # Hand-written lexer, all token types
│   ├── lm-parser/         # Recursive descent parser + AST (M2)
│   ├── lm-types/          # Hindley-Milner inference + effect checking (M3)
│   ├── lm-eval/           # Tree-walking interpreter (M4)
│   ├── lm-cli/            # lmc binary, clap-based CLI
│   └── lm-lsp/            # Language server protocol (M5)
├── examples/              # .lm example programs
└── tests/                 # Integration + snapshot tests
```

## Building

```sh
cargo build            # Build all crates
cargo test             # Run all tests (34 tests)
cargo clippy           # Lint (zero warnings)
```

## Roadmap

- [x] **M1: Diagnostics + Lexer** — Diagnostic framework, hand-written lexer, CLI `tokenize`, 34 tests
- [ ] **M2: Parser + AST** — Recursive descent parser, Pratt parsing for operators, error recovery
- [ ] **M3: Type System** — Hindley-Milner inference, effect checking, exhaustiveness checking
- [ ] **M4: Interpreter** — Tree-walking evaluator, built-in functions, runtime errors
- [ ] **M5: LSP** — Diagnostics, hover types, go-to-definition, VSCode extension
- [ ] **M6: Benchmark** — 30 standard tasks, compare LM vs TypeScript vs Python first-pass rates

## Tech Stack

- Implementation language: Rust (edition 2021)
- Dependencies: `serde`, `serde_json`, `clap`, `insta` (snapshot testing)
- No external lexer generators, no external parser generators — everything hand-written for full control
- First backend: tree-walking interpreter (no bytecode VM, no LLVM)

## License

MIT
