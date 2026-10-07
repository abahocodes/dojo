function maxDepth(root) {
  if (root === null) return 0;
  let depth = 0;
  let level = [root];
  while (level.length > 0) {
    depth++;
    const next = [];
    for (const node of level) {
      if (node.left) next.push(node.left);
      if (node.right) next.push(node.right);
    }
    level = next;
  }
  return depth;
}
