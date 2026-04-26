class Expr:
    pass

class Num(Expr):
    def __init__(self, n):
        self.n = n

class Add(Expr):
    def __init__(self, a, b):
        self.a = a
        self.b = b

class Mul(Expr):
    def __init__(self, a, b):
        self.a = a
        self.b = b

class Neg(Expr):
    def __init__(self, x):
        self.x = x

def eval_expr(expr):
    if isinstance(expr, Num):
        return expr.n
    elif isinstance(expr, Add):
        return eval_expr(expr.a) + eval_expr(expr.b)
    elif isinstance(expr, Mul):
        return eval_expr(expr.a) * eval_expr(expr.b)
    elif isinstance(expr, Neg):
        return -eval_expr(expr.x)

def show(expr):
    if isinstance(expr, Num):
        return str(expr.n)
    elif isinstance(expr, Add):
        return f"({show(expr.a)} + {show(expr.b)})"
    elif isinstance(expr, Mul):
        return f"({show(expr.a)} * {show(expr.b)})"
    elif isinstance(expr, Neg):
        return f"(-({show(expr.x)}))"

# Test eval
print(eval_expr(Num(5)))
print(eval_expr(Add(Num(3), Num(5))))
print(eval_expr(Mul(Num(2), Num(4))))
print(eval_expr(Neg(Num(3))))
print(eval_expr(Neg(Add(Num(2), Num(3)))))

# Test show
print(show(Num(5)))
print(show(Add(Num(3), Num(5))))
print(show(Mul(Num(2), Num(4))))
print(show(Neg(Num(3))))
print(show(Neg(Add(Num(2), Num(3)))))
