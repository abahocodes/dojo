function isUnivalTree(root: TreeNode | null): boolean {
  if (root === null) return true;
  const value = root.val;
  const stack: TreeNode[] = [root];
  while (stack.length > 0) {
    const node = stack.pop()!;
    if (node.val !== value) return false;
    if (node.left) stack.push(node.left);
    if (node.right) stack.push(node.right);
  }
  return true;
}
