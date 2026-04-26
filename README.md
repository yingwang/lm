# LM — A Programming Language Optimized for LLM Code Generation

[中文版](#中文)

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

## Quick Start

```sh
# Build the compiler
cargo build --release

# Install lmc to your PATH
cp target/release/lmc /usr/local/bin/

# Run your first program
lmc run examples/hello.lm
```

Write a program (`hello.lm`):

```lm
io fn main() -> Unit {
    print("Hello, world!")
}

let _ = main();
```

Run it:

```sh
lmc run hello.lm
```

**Learn LM:**
- [Quick Start](docs/quickstart.md) — 5-minute guide for programmers
- [Tutorial](docs/tutorial.md) — Complete language reference
- [LLM Reference](docs/llm-reference.md) — Cheat sheet for LLM agents

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
│   ├── lm-parser/         # Recursive descent parser + AST
│   ├── lm-types/          # Hindley-Milner inference + effect checking
│   ├── lm-eval/           # Tree-walking interpreter
│   ├── lm-cli/            # lmc binary, clap-based CLI
│   └── lm-lsp/            # Language server protocol (planned)
├── examples/              # .lm example programs
└── tests/                 # Integration + snapshot tests
```

## Building

```sh
cargo build            # Build all crates
cargo test             # Run all 182 tests
cargo clippy           # Lint (zero warnings)
```

## Editor Support

A VSCode extension is included at `editors/vscode/`:

```sh
cd editors/vscode
npm install
npm run compile
# Then install the .vsix or run in Extension Development Host
```

Features: syntax highlighting, real-time diagnostics, hover types, go-to-definition, document outline. The extension spawns `lmc lsp` as the language server.

## Benchmark: Does LM Actually Help LLMs Write Better Code?

We ran a blind benchmark comparing LM, TypeScript, and Python. Independent Claude agents generated solutions for each language, seeing only the task description — never the expected output.

### Standard Tasks (30)

```sh
./benchmark/run_benchmark.sh
```

30 tasks covering string processing, math, list operations, ADT/pattern matching, effect system, and error handling. **30/30 pass.**

### Adversarial Tasks (10)

Tasks specifically designed to trigger common LLM mistakes: implicit type conversion, missing null checks, non-exhaustive pattern matches, broken error propagation chains, accidental mutation, and mixed pure/IO code.

**Claude Haiku 4.5** (weakest Claude model):

| Language | Score | Notes |
|----------|:-----:|-------|
| **LM** | **10/10** | All correct |
| TypeScript | 9/10 | Bug in recursive ADT evaluation |
| Python | 9/10 | Same bug as TypeScript |

Both TypeScript and Python failed on the same task: evaluating a recursive expression tree with a `Neg` variant. Haiku forgot to negate in the `Neg` case — `evalExpr(Neg(Num(3)))` returned `8` instead of `-3`.

In LM, the same model got it right. LM's exhaustive pattern matching forced explicit handling of every variant, and the destructuring pattern guided the model toward the correct implementation.

**Claude Opus 4.6** (strongest Claude model): 10/10 in all three languages. The strong model doesn't need guardrails.

### The Documentation Lesson

The more surprising finding: how much LM's documentation matters for LLM correctness.

| LM Reference Version | Score |
|---|:---:|
| v1 (original) | 18/30 |
| v2 (+no nested functions) | 27/30 |
| v3 (+list_map is IO, +len vs str_len) | 29/30 |

Every v1 failure was the same bug: the model defined helper functions inside other functions. LM doesn't support this, but the reference didn't say so. **One line of documentation fixed 12 failures.**

For LLM-targeted languages, documentation completeness is a feature. An undocumented constraint is worse than no constraint — the model assumes the feature exists and writes code that won't compile.

### Reproducing

All blind-generated solutions and runner scripts are in `benchmark/comparison/`. The adversarial task definitions are in `benchmark/comparison/adversarial_tasks.json`.

## Roadmap

- [x] **M1: Diagnostics + Lexer** — Diagnostic framework, hand-written lexer, CLI `tokenize` (34 tests)
- [x] **M2: Parser + AST** — Recursive descent parser, Pratt parsing, error recovery (36 tests)
- [x] **M3: Type System** — Hindley-Milner inference, effect checking, exhaustiveness (53 tests)
- [x] **M4: Interpreter** — Tree-walking evaluator, built-in functions, recursion (24 tests)
- [x] **M5: LSP + Editor** — Language server, VSCode extension with hover/goto-def/symbols (15 tests)
- [x] **M6: Benchmark** — 30/30 standard tasks + 10/10 adversarial tasks pass
- [x] **M7: Cross-language benchmark** — Blind comparison vs TypeScript and Python

**182 tests total, zero clippy warnings.**

## Tech Stack

- Implementation language: Rust (edition 2021)
- Dependencies: `serde`, `serde_json`, `clap`, `insta`, `tower-lsp`, `tokio`
- No external lexer/parser generators — everything hand-written for full control
- Tree-walking interpreter (no bytecode VM, no LLVM)

## License

MIT

---

<a id="中文"></a>
## 中文

# LM — 为 LLM 代码生成优化的编程语言

LM 是一门专为大语言模型写代码而设计的编程语言。每一个设计决策都在消除 LLM 写代码时常犯的错误：没有可变性、没有隐式转换、没有 null、没有异常、没有继承。编译器同时输出人类可读和 JSON 两种格式的诊断信息，LLM 代理可以解析错误并自我修正。

**目标：** 在一组标准编程任务上，Claude 用 LM 写代码的一次通过率显著高于 TypeScript / Python。

### 快速开始

```sh
# 编译
cargo build --release

# 安装 lmc 到 PATH
cp target/release/lmc /usr/local/bin/

# 运行程序
lmc run examples/hello.lm
```

**学习 LM：**
- [快速上手](docs/quickstart.md) — 5 分钟入门（对比 Python/JS）
- [完整教程](docs/tutorial.md) — 语言参考手册
- [LLM 参考](docs/llm-reference.md) — 给 AI 的速查表

### 核心规则

| 规则 | 理由 |
|------|------|
| 完全不可变，没有 `mut` | 消除别名 bug、竞态条件、远距离幽灵操作 |
| `+` 仅用于数字，`++` 用于字符串拼接 | `+` 的含义永远没有歧义 |
| 没有隐式类型转换 | `Int` + `Float` 是编译错误，必须显式转换 |
| 没有 null/nil，用 `Option<T>` | 每个调用点都必须处理空值 |
| 没有异常，用 `Result<T, E>` | 错误路径在类型签名中可见 |
| 没有继承/trait/方法分派 | 只有一种方式：函数 + 模式匹配 |
| 没有宏/反射/元编程 | 所见即所执行 |
| Effect 系统：`pure`（默认）/ `io` | 纯函数不能调用 IO，编译器强制检查 |
| 模式匹配必须穷尽 | 漏了一个分支？编译错误。 |
| Hindley-Milner 类型推导 | 全局类型推导，鼓励显式标注 |

### 语言示例

```lm
// 纯函数，默认 pure
fn add(a: Int, b: Int) -> Int {
    a + b
}

// IO 函数必须显式标注
io fn greet(name: String) -> Unit {
    print("Hello, " ++ name ++ "!")
}

// 代数数据类型 + 穷尽模式匹配
type Shape =
    | Circle(Float)
    | Rect(Float, Float)

fn area(s: Shape) -> Float {
    match s {
        Circle(r) -> 3.14159 * r * r,
        Rect(w, h) -> w * h,
    }
}

// 没有 null，没有异常 — 只有 Option 和 Result
fn safe_div(a: Int, b: Int) -> Result<Int, String> {
    match b {
        0 -> Err("division by zero"),
        _ -> Ok(a / b),
    }
}
```

### 诊断系统

编译器诊断信息同时支持人类可读格式（类似 rustc 的彩色输出）和 JSON 格式（供 LLM 代理和工具链使用）。错误码一旦分配就不会改变语义。

### CLI 命令

```sh
lmc tokenize <file>     # 输出 token 流
lmc parse <file>         # 输出 AST（JSON）
lmc check <file>         # 类型检查，不执行
lmc run <file>           # 类型检查 + 执行
lmc lsp                  # 启动语言服务器
--format=human|json      # 诊断输出格式
```

### 编辑器支持

VSCode 扩展在 `editors/vscode/` 目录，支持语法高亮、实时诊断、悬停查看类型、跳转到定义、文档大纲。

### 基准测试：LM 真的能帮 LLM 写出更好的代码吗？

我们用盲测对比了 LM、TypeScript 和 Python。独立的 Claude agent 只看到任务描述，看不到预期输出。

**Claude Haiku 4.5**（最弱的 Claude 模型）在 10 个对抗性任务上的表现：

| 语言 | 得分 | 说明 |
|------|:----:|------|
| **LM** | **10/10** | 全部正确 |
| TypeScript | 9/10 | 递归 ADT 求值 bug |
| Python | 9/10 | 同样的 bug |

TypeScript 和 Python 都在同一个任务上失败：求值带 `Neg` 变体的递归表达式树。Haiku 忘记取反——`Neg(Num(3))` 返回了 `8` 而不是 `-3`。LM 版本写对了，因为穷尽模式匹配强制处理每个变体。

**文档教训**：LM 的 reference doc 从缺失关键约束到补全，first-pass correctness 从 18/30 跳到 29/30。**一行文档修复了 12 个失败。**

### 技术栈

- 实现语言：Rust
- 手写词法分析器和递归下降语法分析器，完全可控
- Hindley-Milner 类型推导 + effect 系统 + 穷尽性检查
- 树遍历解释器 + LSP 语言服务器

### 进度

- [x] M1：诊断框架 + 词法分析器（34 个测试）
- [x] M2：语法分析器 + AST（36 个测试）
- [x] M3：类型系统 + Effect 检查 + 穷尽性检查（53 个测试）
- [x] M4：树遍历解释器 + 内置函数（24 个测试）
- [x] M5：LSP 语言服务器 + VSCode 扩展（16 个测试）
- [x] M6：基准测试套件 — 30/30 标准 + 10/10 对抗性任务通过
- [x] M7：跨语言盲测 — 对比 TypeScript 和 Python

**182 个测试，全部通过，clippy 零警告。**
