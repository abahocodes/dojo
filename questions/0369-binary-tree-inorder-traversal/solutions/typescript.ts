function inorderTraversal(root: TreeNode | null): number[] {
  const result: number[] = [];
  const stack: TreeNode[] = [];
  let cur: TreeNode | null = root;
  while (cur !== null || stack.length > 0) {
    while (cur !== null) {
      stack.push(cur);
      cur = cur.left;
    }
    const node: TreeNode = stack.pop()!;
    result.push(node.val);
    cur = node.right;
  }
  return result;
}
