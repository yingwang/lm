class Leaf:
    def __init__(self, v):
        self.v = v

class Node:
    def __init__(self, left, right):
        self.left = left
        self.right = right

def depth(t):
    if isinstance(t, Leaf):
        return 1
    return 1 + max(depth(t.left), depth(t.right))

print(depth(Leaf(1)))
print(depth(Node(Leaf(1), Leaf(2))))
print(depth(Node(Node(Leaf(1), Leaf(2)), Leaf(3))))
