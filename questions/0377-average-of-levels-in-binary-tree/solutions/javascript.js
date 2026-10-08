function averageOfLevels(root) {
  const averages = [];
  let level = [root];
  while (level.length > 0) {
    let total = 0;
    const next = [];
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
