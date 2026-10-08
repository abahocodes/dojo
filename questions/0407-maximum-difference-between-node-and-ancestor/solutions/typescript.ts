function maxAncestorDiff(root: TreeNode | null): number {
  if (root === null) return 0;
  let best = 0;
  const stack: [TreeNode, number, number][] = [[root, root.val, root.val]];
  while (stack.length > 0) {
    let [node, lo, hi] = stack.pop()!;
    lo = Math.min(lo, node.val);
    hi = Math.max(hi, node.val);
    best = Math.max(best, hi - lo);
    if (node.left !== null) stack.push([node.left, lo, hi]);
    if (node.right !== null) stack.push([node.right, lo, hi]);
  }
  return best;
}
