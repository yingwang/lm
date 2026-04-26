type Option<T> = { type: "Some"; value: T } | { type: "None" };

function safeHead(lst: number[]): Option<number> {
  if (lst.length === 0) {
    return { type: "None" };
  }
  return { type: "Some", value: lst[0] };
}

function safeMax(lst: number[]): Option<number> {
  if (lst.length === 0) {
    return { type: "None" };
  }
  return { type: "Some", value: Math.max(...lst) };
}

function headsMax(list1: number[], list2: number[]): Option<number> {
  const head1 = safeHead(list1);
  const head2 = safeHead(list2);

  const heads: number[] = [];
  if (head1.type === "Some") heads.push(head1.value);
  if (head2.type === "Some") heads.push(head2.value);

  if (heads.length === 0) {
    return { type: "None" };
  }
  return { type: "Some", value: Math.max(...heads) };
}

function optionToString<T>(opt: Option<T>): string {
  if (opt.type === "Some") {
    return `Some(${opt.value})`;
  }
  return "None";
}

// Test cases
console.log(optionToString(headsMax([5, 1], [3, 2])));
console.log(optionToString(headsMax([], [3])));
console.log(optionToString(headsMax([3], [])));
console.log(optionToString(headsMax([], [])));
