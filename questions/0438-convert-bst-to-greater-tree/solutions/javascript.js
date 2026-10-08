function convertBst(root) {
  let running = 0;
  const stack = [];
  let node = root;
  while (stack.length > 0 || node !== null) {
    while (node !== null) {
      stack.push(node);
      node = node.right;
    }
    node = stack.pop();
    running += node.val;
    node.val = running;
    node = node.left;
  }
  return root;
}
