function getMinimumDifference(root: TreeNode | null): number {
  let best = Infinity;
  let prev: number | null = null;
  const stack: TreeNode[] = [];
  let node = root;
  while (stack.length > 0 || node) {
    while (node) {
      stack.push(node);
      node = node.left;
    }
    const cur = stack.pop()!;
    if (prev !== null) best = Math.min(best, cur.val - prev);
    prev = cur.val;
    node = cur.right;
  }
  return best;
}
