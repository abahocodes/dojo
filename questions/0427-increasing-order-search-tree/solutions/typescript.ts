function increasingBst(root: TreeNode | null): TreeNode | null {
  const dummy = new TreeNode(0);
  let tail: TreeNode = dummy;
  const stack: TreeNode[] = [];
  let node = root;
  while (stack.length > 0 || node) {
    while (node) {
      stack.push(node);
      node = node.left;
    }
    const cur = stack.pop()!;
    cur.left = null;
    tail.right = cur;
    tail = cur;
    node = cur.right;
  }
  return dummy.right;
}
