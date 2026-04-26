type Expr =
  | { tag: "Num"; value: number }
  | { tag: "Add"; a: Expr; b: Expr }
  | { tag: "Mul"; a: Expr; b: Expr }
  | { tag: "Neg"; x: Expr };

function num(n: number): Expr { return { tag: "Num", value: n }; }
function add(a: Expr, b: Expr): Expr { return { tag: "Add", a, b }; }
function mul(a: Expr, b: Expr): Expr { return { tag: "Mul", a, b }; }
function neg(x: Expr): Expr { return { tag: "Neg", x }; }

function eval_expr(e: Expr): number {
  switch (e.tag) {
    case "Num": return e.value;
    case "Add": return eval_expr(e.a) + eval_expr(e.b);
    case "Mul": return eval_expr(e.a) * eval_expr(e.b);
    case "Neg": return -eval_expr(e.x);
  }
}

function show(e: Expr): string {
  switch (e.tag) {
    case "Num": return String(e.value);
    case "Add": return `(${show(e.a)} + ${show(e.b)})`;
    case "Mul": return `(${show(e.a)} * ${show(e.b)})`;
    case "Neg": return `(-(${show(e.x)}))`;
  }
}

// eval tests
console.log(eval_expr(num(5)));
console.log(eval_expr(add(num(3), num(5))));
console.log(eval_expr(neg(num(3))));
console.log(eval_expr(neg(add(num(2), num(3)))));

// show tests
console.log(show(num(5)));
console.log(show(add(num(3), num(5))));
console.log(show(mul(num(2), num(4))));
console.log(show(neg(num(3))));
console.log(show(neg(add(num(2), num(3)))));
