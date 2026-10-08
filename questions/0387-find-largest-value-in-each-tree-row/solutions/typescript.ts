function largestValues(root: TreeNode | null): number[] {
  const result: number[] = [];
  let level: TreeNode[] = root ? [root] : [];
  while (level.length > 0) {
    let best = level[0].val;
    const next: TreeNode[] = [];
    for (const node of level) {
      if (node.val > best) best = node.val;
      if (node.left) next.push(node.left);
      if (node.right) next.push(node.right);
    }
    result.push(best);
    level = next;
  }
  return result;
}
