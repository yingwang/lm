class Leaf:
    pass

class Node:
    def __init__(self, children):
        self.children = children

def tree_depth(tree):
    if isinstance(tree, Leaf):
        return 1
    return 1 + max(tree_depth(c) for c in tree.children)

print(tree_depth(Leaf()))
print(tree_depth(Node([Leaf(), Leaf()])))
print(tree_depth(Node([Node([Leaf()]), Leaf()])))
