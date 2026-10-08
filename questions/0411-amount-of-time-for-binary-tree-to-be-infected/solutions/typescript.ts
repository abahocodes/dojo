function amountOfTime(root: TreeNode | null, start: number): number {
  if (root === null) return 0;
  const parent = new Map<TreeNode, TreeNode | null>([[root, null]]);
  let source: TreeNode = root;
  const stack: TreeNode[] = [root];
  while (stack.length > 0) {
    const node = stack.pop()!;
    if (node.val === start) source = node;
    for (const child of [node.left, node.right]) {
      if (child !== null) {
        parent.set(child, node);
        stack.push(child);
      }
    }
  }
  const seen = new Set<TreeNode>([source]);
  let frontier: TreeNode[] = [source];
  let minutes = -1;
  while (frontier.length > 0) {
    minutes++;
    const next: TreeNode[] = [];
    for (const node of frontier) {
      for (const neighbor of [node.left, node.right, parent.get(node) ?? null]) {
        if (neighbor !== null && !seen.has(neighbor)) {
          seen.add(neighbor);
          next.push(neighbor);
        }
      }
    }
    frontier = next;
  }
  return minutes;
}
