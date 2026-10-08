function lowestCommonAncestor(root, p, q) {
  const lo = Math.min(p, q);
  const hi = Math.max(p, q);
  let node = root;
  while (true) {
    if (hi < node.val) node = node.left;
    else if (lo > node.val) node = node.right;
    else return node.val;
  }
}
