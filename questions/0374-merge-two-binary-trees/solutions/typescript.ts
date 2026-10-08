function mergeTrees(root1: TreeNode | null, root2: TreeNode | null): TreeNode | null {
  if (root1 === null) return root2;
  const stack: [TreeNode, TreeNode | null][] = [[root1, root2]];
  while (stack.length > 0) {
    const [a, b] = stack.pop()!;
    if (b === null) continue;
    a.val += b.val;
    if (a.left === null) a.left = b.left;
    else stack.push([a.left, b.left]);
    if (a.right === null) a.right = b.right;
    else stack.push([a.right, b.right]);
  }
  return root1;
}
