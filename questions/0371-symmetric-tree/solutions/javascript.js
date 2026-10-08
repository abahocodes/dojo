function isSymmetric(root) {
  const stack = [root.left, root.right];
  while (stack.length > 0) {
    const b = stack.pop();
    const a = stack.pop();
    if (a === null && b === null) continue;
    if (a === null || b === null || a.val !== b.val) return false;
    stack.push(a.left, b.right);
    stack.push(a.right, b.left);
  }
  return true;
}
