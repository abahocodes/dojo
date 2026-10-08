function recoverFromPreorder(traversal) {
  const stack = [];
  const n = traversal.length;
  let i = 0;
  while (i < n) {
    let depth = 0;
    while (traversal[i] === "-") {
      depth++;
      i++;
    }
    let value = 0;
    while (i < n && traversal[i] !== "-") {
      value = value * 10 + (traversal.charCodeAt(i) - 48);
      i++;
    }
    const node = new TreeNode(value);
    stack.length = depth;
    if (depth > 0) {
      const parent = stack[depth - 1];
      if (parent.left === null) parent.left = node;
      else parent.right = node;
    }
    stack.push(node);
  }
  return stack[0];
}
