type Tree = { tag: "Leaf" } | { tag: "Node"; children: Tree[] };

function tree_depth(tree: Tree): number {
  if (tree.tag === "Leaf") return 1;
  let maxChild = 0;
  for (const child of tree.children) {
    const d = tree_depth(child);
    if (d > maxChild) maxChild = d;
  }
  return 1 + maxChild;
}

const leaf: Tree = { tag: "Leaf" };
const node1: Tree = { tag: "Node", children: [leaf] };
const node2: Tree = { tag: "Node", children: [node1, leaf] };

console.log(tree_depth(leaf));
console.log(tree_depth(node1));
console.log(tree_depth(node2));
