//! Abstract Syntax Tree types for the LM programming language.
//!
//! Every AST node carries a [`Span`] for source location tracking.
//! All types derive `Debug`, `Clone`, and `Serialize` for inspection
//! and snapshot testing.

use lm_diagnostics::Span;
use serde::Serialize;

/// A complete LM source file: a sequence of top-level declarations.
#[derive(Debug, Clone, Serialize)]
pub struct Program {
    /// The top-level declarations in source order.
    pub decls: Vec<Decl>,
    /// Span covering the entire file.
    pub span: Span,
}

/// A top-level declaration.
#[derive(Debug, Clone, Serialize)]
pub struct Decl {
    /// The kind of declaration.
    pub kind: DeclKind,
    /// Source span covering the entire declaration.
    pub span: Span,
}

/// The different kinds of top-level declarations.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum DeclKind {
    /// A function definition: `fn name(params) -> RetType { body }` or `io fn ...`.
    FnDef {
        /// Function name.
        name: String,
        /// Effect annotation: `pure` (default) or `io`.
        effect: Effect,
        /// Parameter list with optional type annotations.
        params: Vec<Param>,
        /// Optional return type annotation.
        return_type: Option<TypeAnnotation>,
        /// Function body expression.
        body: Expr,
    },
    /// An algebraic data type definition: `type Name = | Variant1(...) | Variant2(...)`.
    TypeDef {
        /// Type name.
        name: String,
        /// Type parameters (e.g., `T` in `type Option<T>`).
        type_params: Vec<String>,
        /// Variant definitions.
        variants: Vec<Variant>,
    },
    /// A top-level let binding: `let name = expr;`.
    LetDef {
        /// Binding name.
        name: String,
        /// Optional type annotation.
        type_annotation: Option<TypeAnnotation>,
        /// Value expression.
        value: Expr,
    },
}

/// Function effect annotation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Effect {
    /// Pure function (default).
    Pure,
    /// IO-performing function.
    Io,
}

/// A function parameter.
#[derive(Debug, Clone, Serialize)]
pub struct Param {
    /// Parameter name.
    pub name: String,
    /// Optional type annotation.
    pub type_annotation: Option<TypeAnnotation>,
    /// Source span.
    pub span: Span,
}

/// A variant in a type definition.
#[derive(Debug, Clone, Serialize)]
pub struct Variant {
    /// Variant name.
    pub name: String,
    /// Optional tuple of field types.
    pub fields: Vec<TypeAnnotation>,
    /// Source span.
    pub span: Span,
}

/// An expression node.
#[derive(Debug, Clone, Serialize)]
pub struct Expr {
    /// The kind of expression.
    pub kind: ExprKind,
    /// Source span.
    pub span: Span,
}

/// The different kinds of expressions.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum ExprKind {
    /// A literal value.
    Literal {
        /// The literal value.
        value: LitValue,
    },
    /// A variable or name reference.
    Ident {
        /// The identifier name.
        name: String,
    },
    /// A binary operation: `lhs op rhs`.
    BinaryOp {
        /// The operator.
        op: BinOp,
        /// Left-hand operand.
        lhs: Box<Expr>,
        /// Right-hand operand.
        rhs: Box<Expr>,
    },
    /// A unary operation: `op expr`.
    UnaryOp {
        /// The operator.
        op: UnOp,
        /// The operand.
        operand: Box<Expr>,
    },
    /// A function call: `callee(args)`.
    FnCall {
        /// The expression being called.
        callee: Box<Expr>,
        /// The argument expressions.
        args: Vec<Expr>,
    },
    /// A let expression: `let x = value; body`.
    LetExpr {
        /// Binding name.
        name: String,
        /// Optional type annotation.
        type_annotation: Option<TypeAnnotation>,
        /// Value expression.
        value: Box<Expr>,
        /// Body expression (the value of the whole let expression).
        body: Box<Expr>,
    },
    /// An if-else expression: `if cond { then } else { else_ }`.
    IfElse {
        /// Condition expression.
        condition: Box<Expr>,
        /// Then branch.
        then_branch: Box<Expr>,
        /// Else branch (mandatory in LM since everything is an expression).
        else_branch: Box<Expr>,
    },
    /// A match expression: `match scrutinee { arms }`.
    Match {
        /// The expression being matched.
        scrutinee: Box<Expr>,
        /// The match arms.
        arms: Vec<MatchArm>,
    },
    /// A block expression: `{ expr1; expr2; ... exprN }`.
    Block {
        /// The expressions in the block, where the last is the value.
        exprs: Vec<Expr>,
    },
    /// A list literal: `[1, 2, 3]` or `[]`.
    ListLiteral {
        /// The elements of the list.
        elements: Vec<Expr>,
    },
    /// Constructing an ADT variant: `Circle(5.0)`, `None`, `Ok(42)`.
    VariantConstruct {
        /// The variant name.
        name: String,
        /// Arguments (empty for unit variants like `None`).
        args: Vec<Expr>,
    },
    /// A parse error placeholder so we can continue past errors.
    Error,
}

/// Literal values.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "kind", content = "value")]
pub enum LitValue {
    /// Integer literal.
    Int(i64),
    /// Floating-point literal.
    Float(f64),
    /// String literal (the content between quotes, with escapes processed).
    String(String),
    /// Boolean literal.
    Bool(bool),
}

/// Binary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum BinOp {
    /// `+`
    Add,
    /// `-`
    Sub,
    /// `*`
    Mul,
    /// `/`
    Div,
    /// `%`
    Mod,
    /// `++` (string concatenation)
    Concat,
    /// `==`
    Eq,
    /// `!=`
    Ne,
    /// `<`
    Lt,
    /// `<=`
    Le,
    /// `>`
    Gt,
    /// `>=`
    Ge,
    /// `&&`
    And,
    /// `||`
    Or,
}

/// Unary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum UnOp {
    /// `!` (logical not)
    Not,
    /// `-` (numeric negation)
    Neg,
}

/// A match arm: `pattern -> expression`.
#[derive(Debug, Clone, Serialize)]
pub struct MatchArm {
    /// The pattern.
    pub pattern: Pattern,
    /// The body expression.
    pub body: Expr,
    /// Source span.
    pub span: Span,
}

/// A pattern in a match arm.
#[derive(Debug, Clone, Serialize)]
pub struct Pattern {
    /// The kind of pattern.
    pub kind: PatternKind,
    /// Source span.
    pub span: Span,
}

/// The different kinds of patterns.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum PatternKind {
    /// Match against a literal value.
    Literal {
        /// The literal value.
        value: LitValue,
    },
    /// Bind to a variable name.
    Ident {
        /// The variable name.
        name: String,
    },
    /// Match a variant with sub-patterns: `Circle(r)`, `Ok(value)`.
    Variant {
        /// The variant name.
        name: String,
        /// Sub-patterns for the variant fields.
        fields: Vec<Pattern>,
    },
    /// Wildcard pattern: `_`.
    Wildcard,
}

/// A type annotation.
#[derive(Debug, Clone, Serialize)]
pub struct TypeAnnotation {
    /// The kind of type.
    pub kind: TypeKind,
    /// Source span.
    pub span: Span,
}

/// The different kinds of type annotations.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum TypeKind {
    /// A simple named type: `Int`, `Float`, `Bool`, `String`, `Unit`.
    Name {
        /// The type name.
        name: String,
    },
    /// A parameterized type: `Option<Int>`, `Result<Int, String>`.
    App {
        /// The base type name.
        name: String,
        /// Type arguments.
        args: Vec<TypeAnnotation>,
    },
    /// A function type: `(Int, Int) -> Int`.
    Fn {
        /// Parameter types.
        params: Vec<TypeAnnotation>,
        /// Return type.
        ret: Box<TypeAnnotation>,
    },
}
