function findTilt(root) {
  if (root === null) return 0;
  const order = [];
  const stack = [root];
  while (stack.length > 0) {
    const node = stack.pop();
    order.push(node);
    if (node.left) stack.push(node.left);
    if (node.right) stack.push(node.right);
  }
  const subtreeSum = new Map();
  let tilt = 0;
  for (let i = order.length - 1; i >= 0; i--) {
    const node = order[i];
    const left = node.left ? subtreeSum.get(node.left) : 0;
    const right = node.right ? subtreeSum.get(node.right) : 0;
    tilt += Math.abs(left - right);
    subtreeSum.set(node, node.val + left + right);
  }
  return tilt;
}
