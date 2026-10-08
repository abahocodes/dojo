function invertTree(root) {
  const stack = [root];
  while (stack.length > 0) {
    const node = stack.pop();
    if (node !== null) {
      const left = node.left;
      node.left = node.right;
      node.right = left;
      stack.push(node.left, node.right);
    }
  }
  return root;
}
