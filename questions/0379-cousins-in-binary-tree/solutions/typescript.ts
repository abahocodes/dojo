function isCousins(root: TreeNode | null, x: number, y: number): boolean {
  if (root === null) return false;
  let level: TreeNode[] = [root];
  while (level.length > 0) {
    let parentX: TreeNode | null = null;
    let parentY: TreeNode | null = null;
    const next: TreeNode[] = [];
    for (const node of level) {
      for (const child of [node.left, node.right]) {
        if (child === null) continue;
        if (child.val === x) parentX = node;
        else if (child.val === y) parentY = node;
        next.push(child);
      }
    }
    if (parentX !== null && parentY !== null) return parentX !== parentY;
    if (parentX !== null || parentY !== null) return false;
    level = next;
  }
  return false;
}
