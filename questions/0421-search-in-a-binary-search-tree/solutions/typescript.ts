function searchBst(root: TreeNode | null, val: number): TreeNode | null {
  let node = root;
  while (node !== null && node.val !== val) {
    node = val < node.val ? node.left : node.right;
  }
  return node;
}
