function boundaryOfBinaryTree(root: TreeNode | null): number[] {
  if (root === null) return [];
  const isLeaf = (node: TreeNode): boolean => node.left === null && node.right === null;
  const result: number[] = [root.val];
  if (isLeaf(root)) return result;

  let node: TreeNode | null = root.left;
  while (node !== null && !isLeaf(node)) {
    result.push(node.val);
    node = node.left !== null ? node.left : node.right;
  }

  const stack: TreeNode[] = [root];
  while (stack.length > 0) {
    const cur = stack.pop()!;
    if (isLeaf(cur)) {
      result.push(cur.val);
      continue;
    }
    if (cur.right !== null) stack.push(cur.right);
    if (cur.left !== null) stack.push(cur.left);
  }

  const right: number[] = [];
  node = root.right;
  while (node !== null && !isLeaf(node)) {
    right.push(node.val);
    node = node.right !== null ? node.right : node.left;
  }
  for (let i = right.length - 1; i >= 0; i--) result.push(right[i]);
  return result;
}
