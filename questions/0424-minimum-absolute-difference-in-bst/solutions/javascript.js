function getMinimumDifference(root) {
  let best = Infinity;
  let prev = null;
  const stack = [];
  let node = root;
  while (stack.length > 0 || node) {
    while (node) {
      stack.push(node);
      node = node.left;
    }
    node = stack.pop();
    if (prev !== null) best = Math.min(best, node.val - prev);
    prev = node.val;
    node = node.right;
  }
  return best;
}
