function diameterOfBinaryTree(root) {
  const order = [];
  const stack = [root];
  while (stack.length > 0) {
    const node = stack.pop();
    order.push(node);
    if (node.left) stack.push(node.left);
    if (node.right) stack.push(node.right);
  }
  const height = new Map();
  let best = 0;
  for (let i = order.length - 1; i >= 0; i--) {
    const node = order[i];
    const left = node.left ? height.get(node.left) : 0;
    const right = node.right ? height.get(node.right) : 0;
    best = Math.max(best, left + right);
    height.set(node, 1 + Math.max(left, right));
  }
  return best;
}
