function findSecondMinimumValue(root: TreeNode | null): number {
  if (root === null) return -1;
  const smallest = root.val;
  let best = -1;
  const stack: TreeNode[] = [root];
  while (stack.length > 0) {
    const node = stack.pop()!;
    if (node.val > smallest) {
      if (best === -1 || node.val < best) best = node.val;
      continue;
    }
    if (node.left && node.right) {
      stack.push(node.left);
      stack.push(node.right);
    }
  }
  return best;
}
