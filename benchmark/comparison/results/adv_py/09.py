def process_data(lst):
    filtered = [x for x in lst if x >= 0]
    doubled = [x * 2 for x in filtered]
    return sum(doubled)

def run(lst):
    print(f"Total: {process_data(lst)}")

run([1, 2, 3])
run([-1, -2])
run([5, -3, 5])
