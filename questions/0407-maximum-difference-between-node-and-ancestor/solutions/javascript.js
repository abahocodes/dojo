function maxAncestorDiff(root) {
  let best = 0;
  const stack = [[root, root.val, root.val]];
  while (stack.length > 0) {
    let [node, lo, hi] = stack.pop();
    lo = Math.min(lo, node.val);
    hi = Math.max(hi, node.val);
    best = Math.max(best, hi - lo);
    if (node.left !== null) stack.push([node.left, lo, hi]);
    if (node.right !== null) stack.push([node.right, lo, hi]);
  }
  return best;
}
