function isEvenOddTree(root: TreeNode): boolean {
  let level: TreeNode[] = [root];
  let depth = 0;
  while (level.length > 0) {
    const evenLevel = depth % 2 === 0;
    let prev: number | null = null;
    const next: TreeNode[] = [];
    for (const node of level) {
      const v = node.val;
      if (evenLevel) {
        if (v % 2 === 0 || (prev !== null && v <= prev)) return false;
      } else {
        if (v % 2 === 1 || (prev !== null && v >= prev)) return false;
      }
      prev = v;
      if (node.left) next.push(node.left);
      if (node.right) next.push(node.right);
    }
    level = next;
    depth++;
  }
  return true;
}
