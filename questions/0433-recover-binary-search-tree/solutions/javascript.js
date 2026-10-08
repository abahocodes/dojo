function recoverTree(root) {
  let first = null;
  let second = null;
  let prev = null;
  const stack = [];
  let node = root;
  while (stack.length > 0 || node !== null) {
    while (node !== null) {
      stack.push(node);
      node = node.left;
    }
    node = stack.pop();
    if (prev !== null && prev.val > node.val) {
      if (first === null) first = prev;
      second = node;
    }
    prev = node;
    node = node.right;
  }
  const tmp = first.val;
  first.val = second.val;
  second.val = tmp;
  return root;
}
