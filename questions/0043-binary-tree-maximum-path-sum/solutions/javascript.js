function maxPathSum(root) {
  const order = [];
  const stack = [root];
  while (stack.length > 0) {
    const node = stack.pop();
    order.push(node);
    if (node.left !== null) stack.push(node.left);
    if (node.right !== null) stack.push(node.right);
  }

  const gain = new Map();
  let best = root.val;
  for (let i = order.length - 1; i >= 0; i--) {
    const node = order[i];
    const left = Math.max(gain.get(node.left) ?? 0, 0);
    const right = Math.max(gain.get(node.right) ?? 0, 0);
    best = Math.max(best, node.val + left + right);
    gain.set(node, node.val + Math.max(left, right));
  }
  return best;
}
