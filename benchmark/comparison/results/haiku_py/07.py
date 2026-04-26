def safe_head(lst):
    if len(lst) > 0:
        return f"Some({lst[0]})"
    else:
        return "None"

def safe_max(lst):
    if len(lst) > 0:
        return f"Some({max(lst)})"
    else:
        return "None"

def heads_max(lst1, lst2):
    head1 = safe_head(lst1)
    head2 = safe_head(lst2)

    # Extract values
    val1 = None
    val2 = None

    if head1.startswith("Some"):
        val1 = int(head1[5:-1])
    if head2.startswith("Some"):
        val2 = int(head2[5:-1])

    # Return max
    if val1 is not None and val2 is not None:
        return f"Some({max(val1, val2)})"
    elif val1 is not None:
        return f"Some({val1})"
    elif val2 is not None:
        return f"Some({val2})"
    else:
        return "None"

# Test
print(heads_max([5, 1], [3, 2]))
print(heads_max([], [3]))
print(heads_max([3], []))
print(heads_max([], []))
