function maxDepth(root: TreeNode | null): number {
  if (root === null) return 0;
  let depth = 0;
  let level: TreeNode[] = [root];
  while (level.length > 0) {
    depth++;
    const nxt: TreeNode[] = [];
    for (const node of level) {
      if (node.left) nxt.push(node.left);
      if (node.right) nxt.push(node.right);
    }
    level = nxt;
  }
  return depth;
}
