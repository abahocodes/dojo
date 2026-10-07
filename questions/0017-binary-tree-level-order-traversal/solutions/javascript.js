function levelOrder(root) {
  if (root === null) return [];
  const result = [];
  let level = [root];
  while (level.length > 0) {
    const values = [];
    const next = [];
    for (const node of level) {
      values.push(node.val);
      if (node.left) next.push(node.left);
      if (node.right) next.push(node.right);
    }
    result.push(values);
    level = next;
  }
  return result;
}
