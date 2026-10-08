function findTarget(root, k) {
  const values = [];
  const stack = [];
  let node = root;
  while (stack.length > 0 || node) {
    while (node) {
      stack.push(node);
      node = node.left;
    }
    node = stack.pop();
    values.push(node.val);
    node = node.right;
  }

  let i = 0;
  let j = values.length - 1;
  while (i < j) {
    const s = values[i] + values[j];
    if (s === k) return true;
    if (s < k) i++;
    else j--;
  }
  return false;
}
