function minDepth(root) {
  if (root === null) return 0;
  let level = [root];
  let depth = 1;
  while (level.length > 0) {
    const next = [];
    for (const node of level) {
      if (node.left === null && node.right === null) return depth;
      if (node.left) next.push(node.left);
      if (node.right) next.push(node.right);
    }
    level = next;
    depth++;
  }
  return 0;
}
