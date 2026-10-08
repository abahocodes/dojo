function leafSimilar(root1: TreeNode | null, root2: TreeNode | null): boolean {
  const leaves = (root: TreeNode | null): number[] => {
    const out: number[] = [];
    const stack: TreeNode[] = root ? [root] : [];
    while (stack.length > 0) {
      const node = stack.pop()!;
      if (!node.left && !node.right) {
        out.push(node.val);
        continue;
      }
      if (node.right) stack.push(node.right);
      if (node.left) stack.push(node.left);
    }
    return out;
  };
  const a = leaves(root1);
  const b = leaves(root2);
  if (a.length !== b.length) return false;
  for (let i = 0; i < a.length; i++) {
    if (a[i] !== b[i]) return false;
  }
  return true;
}
