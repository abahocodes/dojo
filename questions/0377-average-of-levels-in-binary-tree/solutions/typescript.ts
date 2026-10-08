function averageOfLevels(root: TreeNode | null): number[] {
  if (root === null) return [];
  const averages: number[] = [];
  let level: TreeNode[] = [root];
  while (level.length > 0) {
    let total = 0;
    const next: TreeNode[] = [];
    for (const node of level) {
      total += node.val;
      if (node.left) next.push(node.left);
      if (node.right) next.push(node.right);
    }
    averages.push(total / level.length);
    level = next;
  }
  return averages;
}
