function zigzagLevelOrder(root) {
  const result = [];
  let level = root ? [root] : [];
  let leftToRight = true;
  while (level.length > 0) {
    const values = level.map((node) => node.val);
    if (!leftToRight) values.reverse();
    result.push(values);
    const next = [];
    for (const node of level) {
      if (node.left) next.push(node.left);
      if (node.right) next.push(node.right);
    }
    level = next;
    leftToRight = !leftToRight;
  }
  return result;
}
