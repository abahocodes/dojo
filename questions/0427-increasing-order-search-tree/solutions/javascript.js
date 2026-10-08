function increasingBst(root) {
  const dummy = new TreeNode(0);
  let tail = dummy;
  const stack = [];
  let node = root;
  while (stack.length > 0 || node) {
    while (node) {
      stack.push(node);
      node = node.left;
    }
    node = stack.pop();
    node.left = null;
    tail.right = node;
    tail = node;
    node = node.right;
  }
  return dummy.right;
}
