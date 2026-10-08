function averageOfSubtree(root) {
  const order = [];
  const stack = [root];
  while (stack.length > 0) {
    const node = stack.pop();
    order.push(node);
    if (node.left) stack.push(node.left);
    if (node.right) stack.push(node.right);
  }
  const sums = new Map();
  const sizes = new Map();
  let count = 0;
  for (let i = order.length - 1; i >= 0; i--) {
    const node = order[i];
    let s = node.val;
    let c = 1;
    for (const child of [node.left, node.right]) {
      if (child) {
        s += sums.get(child);
        c += sizes.get(child);
      }
    }
    sums.set(node, s);
    sizes.set(node, c);
    if (Math.floor(s / c) === node.val) count++;
  }
  return count;
}
