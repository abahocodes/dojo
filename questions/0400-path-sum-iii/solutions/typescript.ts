function pathSumCount(root: TreeNode | null, targetSum: number): number {
  if (root === null) return 0;
  const seen = new Map<number, number>([[0, 1]]); // prefix sums on the current root-to-node path
  let count = 0;
  // Each entry: [node, prefix sum above it, leaving?]
  const stack: Array<[TreeNode, number, boolean]> = [[root, 0, false]];
  while (stack.length > 0) {
    const [node, before, leaving] = stack.pop()!;
    const prefix = before + node.val;
    if (leaving) {
      seen.set(prefix, seen.get(prefix)! - 1);
      continue;
    }
    count += seen.get(prefix - targetSum) ?? 0;
    seen.set(prefix, (seen.get(prefix) ?? 0) + 1);
    stack.push([node, before, true]);
    if (node.right !== null) stack.push([node.right, prefix, false]);
    if (node.left !== null) stack.push([node.left, prefix, false]);
  }
  return count;
}
