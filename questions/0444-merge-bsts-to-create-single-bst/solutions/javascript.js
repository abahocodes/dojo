function canMerge(trees) {
  const roots = new Map();
  const leafValues = new Set();
  for (const t of trees) {
    roots.set(t.val, t);
    if (t.left) leafValues.add(t.left.val);
    if (t.right) leafValues.add(t.right.val);
  }

  const candidates = trees.filter((t) => !leafValues.has(t.val));
  if (candidates.length !== 1) return null;
  const root = candidates[0];
  roots.delete(root.val);

  // Iterative DFS carrying the open interval (lo, hi) each node must fit in.
  const stack = [[root, 0, 2 ** 31]];
  while (stack.length > 0) {
    const [node, lo, hi] = stack.pop();
    if (node.val <= lo || node.val >= hi) return null;
    if (node.left === null && node.right === null && roots.has(node.val)) {
      const sub = roots.get(node.val);
      roots.delete(node.val);
      node.left = sub.left;
      node.right = sub.right;
    }
    if (node.left !== null) stack.push([node.left, lo, node.val]);
    if (node.right !== null) stack.push([node.right, node.val, hi]);
  }

  return roots.size === 0 ? root : null;
}
