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

def eval_expr(e):
    if isinstance(e, Num):
        return e.n
    elif isinstance(e, Add):
        return eval_expr(e.a) + eval_expr(e.b)
    elif isinstance(e, Mul):
        return eval_expr(e.a) * eval_expr(e.b)
    elif isinstance(e, Neg):
        return -eval_expr(e.x)

def show(e):
    if isinstance(e, Num):
        return str(e.n)
    elif isinstance(e, Add):
        return f"({show(e.a)} + {show(e.b)})"
    elif isinstance(e, Mul):
        return f"({show(e.a)} * {show(e.b)})"
    elif isinstance(e, Neg):
        return f"(-({show(e.x)}))"

eval_cases = [Num(5), Add(Num(3), Num(5)), Neg(Num(3)), Neg(Add(Num(2), Num(3)))]
for e in eval_cases:
    print(eval_expr(e))

show_cases = [Num(5), Add(Num(3), Num(5)), Mul(Num(2), Num(4)), Neg(Num(3)), Neg(Add(Num(2), Num(3)))]
for e in show_cases:
    print(show(e))
