function findSecondMinimumValue(root) {
  const smallest = root.val;
  let best = -1;
  const stack = [root];
  while (stack.length > 0) {
    const node = stack.pop();
    if (node.val > smallest) {
      if (best === -1 || node.val < best) best = node.val;
      continue;
    }
    if (node.left) {
      stack.push(node.left);
      stack.push(node.right);
    }
  }
  return best;
}
