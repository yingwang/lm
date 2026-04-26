type Tree =
  | { type: "Leaf"; value: number }
  | { type: "Node"; left: Tree; right: Tree };

function Leaf(v: number): Tree { return { type: "Leaf", value: v }; }
function Node(left: Tree, right: Tree): Tree { return { type: "Node", left, right }; }

function depth(t: Tree): number {
  if (t.type === "Leaf") return 1;
  return 1 + Math.max(depth(t.left), depth(t.right));
}

console.log(depth(Leaf(1)));
console.log(depth(Node(Leaf(1), Leaf(2))));
console.log(depth(Node(Node(Leaf(1), Leaf(2)), Leaf(3))));
