type Expr =
  | { type: "Lit"; value: number }
  | { type: "Add"; a: Expr; b: Expr }
  | { type: "Mul"; a: Expr; b: Expr };

function Lit(n: number): Expr { return { type: "Lit", value: n }; }
function Add(a: Expr, b: Expr): Expr { return { type: "Add", a, b }; }
function Mul(a: Expr, b: Expr): Expr { return { type: "Mul", a, b }; }

function eval_expr(e: Expr): number {
  switch (e.type) {
    case "Lit": return e.value;
    case "Add": return eval_expr(e.a) + eval_expr(e.b);
    case "Mul": return eval_expr(e.a) * eval_expr(e.b);
  }
}

function describe(e: Expr): string {
  switch (e.type) {
    case "Lit": return String(e.value);
    case "Add": return `${describe(e.a)} + ${describe(e.b)}`;
    case "Mul": return `${describe(e.a)} * ${describe(e.b)}`;
  }
}

console.log(eval_expr(Lit(5)));
console.log(eval_expr(Add(Lit(3), Lit(4))));
console.log(eval_expr(Mul(Lit(3), Lit(4))));
console.log(describe(Lit(5)));
console.log(describe(Add(Lit(3), Lit(4))));
console.log(describe(Mul(Lit(3), Lit(4))));
