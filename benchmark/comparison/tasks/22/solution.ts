type Expr =
  | { tag: "Lit"; value: number }
  | { tag: "Add"; left: Expr; right: Expr }
  | { tag: "Mul"; left: Expr; right: Expr };

function evaluate(expr: Expr): number {
  switch (expr.tag) {
    case "Lit":
      return expr.value;
    case "Add":
      return evaluate(expr.left) + evaluate(expr.right);
    case "Mul":
      return evaluate(expr.left) * evaluate(expr.right);
  }
}

function show(expr: Expr): string {
  switch (expr.tag) {
    case "Lit":
      return String(expr.value);
    case "Add":
      return `${show(expr.left)} + ${show(expr.right)}`;
    case "Mul":
      return `${show(expr.left)} * ${show(expr.right)}`;
  }
}

const lit3: Expr = { tag: "Lit", value: 3 };
const lit4: Expr = { tag: "Lit", value: 4 };
const lit5: Expr = { tag: "Lit", value: 5 };

console.log(evaluate(lit5));
console.log(evaluate({ tag: "Add", left: lit3, right: lit4 }));
console.log(evaluate({ tag: "Mul", left: lit3, right: lit4 }));
console.log(evaluate({ tag: "Add", left: { tag: "Lit", value: 2 }, right: lit3 }));
console.log(show({ tag: "Add", left: lit3, right: lit4 }));
console.log(show({ tag: "Mul", left: lit3, right: lit4 }));
