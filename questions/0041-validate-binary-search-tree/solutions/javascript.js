function isValidBst(root) {
  const stack = [];
  let prev = null;
  let node = root;
  while (stack.length > 0 || node !== null) {
    while (node !== null) {
      stack.push(node);
      node = node.left;
    }
    node = stack.pop();
    if (prev !== null && node.val <= prev) return false;
    prev = node.val;
    node = node.right;
  }
  return true;
}
