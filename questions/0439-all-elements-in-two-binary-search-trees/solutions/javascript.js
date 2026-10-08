function getAllElements(root1, root2) {
  function inorder(root) {
    const values = [];
    const stack = [];
    let node = root;
    while (stack.length > 0 || node !== null) {
      while (node !== null) {
        stack.push(node);
        node = node.left;
      }
      node = stack.pop();
      values.push(node.val);
      node = node.right;
    }
    return values;
  }
  const a = inorder(root1);
  const b = inorder(root2);
  const merged = [];
  let i = 0;
  let j = 0;
  while (i < a.length && j < b.length) {
    if (a[i] <= b[j]) merged.push(a[i++]);
    else merged.push(b[j++]);
  }
  while (i < a.length) merged.push(a[i++]);
  while (j < b.length) merged.push(b[j++]);
  return merged;
}
