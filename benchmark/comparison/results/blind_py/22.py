class Lit:
    def __init__(self, n):
        self.n = n

class Add:
    def __init__(self, a, b):
        self.a = a
        self.b = b

class Mul:
    def __init__(self, a, b):
        self.a = a
        self.b = b

def eval_expr(e):
    if isinstance(e, Lit):
        return e.n
    elif isinstance(e, Add):
        return eval_expr(e.a) + eval_expr(e.b)
    elif isinstance(e, Mul):
        return eval_expr(e.a) * eval_expr(e.b)

def describe(e):
    if isinstance(e, Lit):
        return str(e.n)
    elif isinstance(e, Add):
        return f"{describe(e.a)} + {describe(e.b)}"
    elif isinstance(e, Mul):
        return f"{describe(e.a)} * {describe(e.b)}"

for e in [Lit(5), Add(Lit(3), Lit(4)), Mul(Lit(3), Lit(4))]:
    print(eval_expr(e))
    print(describe(e))
