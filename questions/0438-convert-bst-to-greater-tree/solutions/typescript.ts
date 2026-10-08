function convertBst(root: TreeNode | null): TreeNode | null {
  let running = 0;
  const stack: TreeNode[] = [];
  let node: TreeNode | null = root;
  while (stack.length > 0 || node !== null) {
    while (node !== null) {
      stack.push(node);
      node = node.right;
    }
    const top = stack.pop()!;
    running += top.val;
    top.val = running;
    node = top.left;
  }
  return root;
}
