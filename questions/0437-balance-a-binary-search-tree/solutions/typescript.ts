function balanceBst(root: TreeNode | null): TreeNode | null {
  const values: number[] = [];
  const stack: TreeNode[] = [];
  let node: TreeNode | null = root;
  while (stack.length > 0 || node !== null) {
    while (node !== null) {
      stack.push(node);
      node = node.left;
    }
    const top = stack.pop()!;
    values.push(top.val);
    node = top.right;
  }
  function build(lo: number, hi: number): TreeNode | null {
    if (lo > hi) return null;
    const mid = Math.floor((lo + hi) / 2);
    return new TreeNode(values[mid], build(lo, mid - 1), build(mid + 1, hi));
  }
  return build(0, values.length - 1);
}
