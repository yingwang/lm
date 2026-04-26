class Some:
    def __init__(self, v):
        self.v = v

class NoneOpt:
    pass

def unwrap_or(opt, default):
    if isinstance(opt, Some):
        return opt.v
    return default

print(unwrap_or(Some(42), 0))
print(unwrap_or(NoneOpt(), 0))
print(unwrap_or(Some(-1), 99))
print(unwrap_or(NoneOpt(), -1))
