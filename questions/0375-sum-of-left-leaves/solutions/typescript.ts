function sumOfLeftLeaves(root: TreeNode | null): number {
  if (root === null) return 0;
  let total = 0;
  const stack: [TreeNode, boolean][] = [[root, false]];
  while (stack.length > 0) {
    const [node, isLeft] = stack.pop()!;
    if (node.left === null && node.right === null) {
      if (isLeft) total += node.val;
      continue;
    }
    if (node.left) stack.push([node.left, true]);
    if (node.right) stack.push([node.right, false]);
  }
  return total;
}
