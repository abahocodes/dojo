function kthSmallest(root, k) {
  const stack = [];
  let node = root;
  while (true) {
    while (node !== null) {
      stack.push(node);
      node = node.left;
    }
    node = stack.pop();
    k -= 1;
    if (k === 0) return node.val;
    node = node.right;
  }
}
