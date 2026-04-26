def process_data(lst):
    # Filter negatives
    filtered = [x for x in lst if x >= 0]
    # Double
    doubled = [x * 2 for x in filtered]
    # Sum
    total = sum(doubled)
    return total

def run(lst):
    result = process_data(lst)
    print(f"Total: {result}")

# Test
run([1, 2, 3])
run([-1, -2])
run([5, -3, 5])
