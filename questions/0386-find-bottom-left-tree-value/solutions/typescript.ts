function findBottomLeftValue(root: TreeNode | null): number {
  if (root === null) return 0;
  const queue: TreeNode[] = [root];
  let node: TreeNode = root;
  for (let head = 0; head < queue.length; head++) {
    node = queue[head];
    if (node.right) queue.push(node.right);
    if (node.left) queue.push(node.left);
  }
  return node.val;
}
