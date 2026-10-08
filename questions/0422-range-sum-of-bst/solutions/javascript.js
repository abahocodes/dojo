function rangeSumBst(root, low, high) {
  let total = 0;
  const stack = root ? [root] : [];
  while (stack.length > 0) {
    const node = stack.pop();
    if (node.val < low) {
      if (node.right) stack.push(node.right);
    } else if (node.val > high) {
      if (node.left) stack.push(node.left);
    } else {
      total += node.val;
      if (node.left) stack.push(node.left);
      if (node.right) stack.push(node.right);
    }
  }
  return total;
}
