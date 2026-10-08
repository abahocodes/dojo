function largestValues(root) {
  const result = [];
  let level = root ? [root] : [];
  while (level.length > 0) {
    let best = level[0].val;
    const next = [];
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
