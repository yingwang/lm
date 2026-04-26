class Lit:
    def __init__(self, value):
        self.value = value

class Add:
    def __init__(self, left, right):
        self.left = left
        self.right = right

class Mul:
    def __init__(self, left, right):
        self.left = left
        self.right = right

def eval_expr(expr):
    if isinstance(expr, Lit):
        return expr.value
    elif isinstance(expr, Add):
        return eval_expr(expr.left) + eval_expr(expr.right)
    elif isinstance(expr, Mul):
        return eval_expr(expr.left) * eval_expr(expr.right)

def show_expr(expr):
    if isinstance(expr, Lit):
        return str(expr.value)
    elif isinstance(expr, Add):
        return f"{show_expr(expr.left)} + {show_expr(expr.right)}"
    elif isinstance(expr, Mul):
        return f"{show_expr(expr.left)} * {show_expr(expr.right)}"

print(eval_expr(Lit(5)))
print(eval_expr(Add(Lit(3), Lit(4))))
print(eval_expr(Mul(Lit(3), Lit(4))))
print(eval_expr(Add(Lit(2), Lit(3))))
print(show_expr(Add(Lit(3), Lit(4))))
print(show_expr(Mul(Lit(3), Lit(4))))
