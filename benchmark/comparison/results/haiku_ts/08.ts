type Expr =
  | { type: "Num"; value: number }
  | { type: "Add"; a: Expr; b: Expr }
  | { type: "Mul"; a: Expr; b: Expr }
  | { type: "Neg"; x: Expr };

function evalExpr(expr: Expr): number {
  switch (expr.type) {
    case "Num":
      return expr.value;
    case "Add":
      return evalExpr(expr.a) + evalExpr(expr.b);
    case "Mul":
      return evalExpr(expr.a) * evalExpr(expr.b);
    case "Neg":
      return -evalExpr(expr.x);
  }
}

function show(expr: Expr): string {
  switch (expr.type) {
    case "Num":
      return expr.value.toString();
    case "Add":
      return `(${show(expr.a)} + ${show(expr.b)})`;
    case "Mul":
      return `(${show(expr.a)} * ${show(expr.b)})`;
    case "Neg":
      return `(-(${show(expr.x)}))`;
  }
}

// Test cases
const tests: Expr[] = [
  { type: "Num", value: 5 },
  { type: "Add", a: { type: "Num", value: 3 }, b: { type: "Num", value: 5 } },
  { type: "Mul", a: { type: "Num", value: 2 }, b: { type: "Num", value: 4 } },
  { type: "Neg", x: { type: "Num", value: 3 } },
  { type: "Neg", x: { type: "Add", a: { type: "Num", value: 2 }, b: { type: "Num", value: 3 } } },
];

for (const expr of tests) {
  console.log(evalExpr(expr));
}

for (const expr of tests) {
  console.log(show(expr));
}
