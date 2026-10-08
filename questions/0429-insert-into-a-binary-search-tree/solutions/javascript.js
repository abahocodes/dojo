function insertIntoBst(root, val) {
  const node = new TreeNode(val);
  if (root === null) return node;
  let cur = root;
  while (true) {
    if (val < cur.val) {
      if (cur.left === null) {
        cur.left = node;
        return root;
      }
      cur = cur.left;
    } else {
      if (cur.right === null) {
        cur.right = node;
        return root;
      }
      cur = cur.right;
    }
  }
}
