def unwrap_or(opt, default):
    if opt is None:
        return default
    return opt

print(unwrap_or(42, 0))
print(unwrap_or(None, 0))
print(unwrap_or(-1, 0))
print(unwrap_or(None, -1))
