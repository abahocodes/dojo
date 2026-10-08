function sumRootToLeaf(root: TreeNode | null): number {
  let total = 0;
  const nodes: TreeNode[] = root ? [root] : [];
  const prefixes: number[] = root ? [0] : [];
  while (nodes.length > 0) {
    const node = nodes.pop()!;
    const value = prefixes.pop()! * 2 + node.val;
    if (!node.left && !node.right) {
      total += value;
      continue;
    }
    if (node.left) { nodes.push(node.left); prefixes.push(value); }
    if (node.right) { nodes.push(node.right); prefixes.push(value); }
  }
  return total;
}
