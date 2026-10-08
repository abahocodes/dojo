function isUnivalTree(root) {
  const value = root.val;
  const stack = [root];
  while (stack.length > 0) {
    const node = stack.pop();
    if (node.val !== value) return false;
    if (node.left) stack.push(node.left);
    if (node.right) stack.push(node.right);
  }
  return true;
}
