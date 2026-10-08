function countNodes(root: TreeNode | null): number {
  const leftDepth = (node: TreeNode | null): number => {
    let depth = 0;
    while (node) {
      depth++;
      node = node.left;
    }
    return depth;
  };
  let count = 0;
  let node = root;
  while (node) {
    const lh = leftDepth(node.left);
    const rh = leftDepth(node.right);
    if (lh === rh) {
      count += 2 ** lh;
      node = node.right;
    } else {
      count += 2 ** rh;
      node = node.left;
    }
  }
  return count;
}
