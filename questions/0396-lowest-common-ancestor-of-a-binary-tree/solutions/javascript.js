function lowestCommonAncestor(root, p, q) {
  const parent = new Map([[root.val, null]]);
  const stack = [root];
  while (stack.length > 0 && !(parent.has(p) && parent.has(q))) {
    const node = stack.pop();
    for (const child of [node.left, node.right]) {
      if (child !== null) {
        parent.set(child.val, node.val);
        stack.push(child);
      }
    }
  }
  const ancestors = new Set();
  for (let v = p; v !== null; v = parent.get(v)) ancestors.add(v);
  let v = q;
  while (!ancestors.has(v)) v = parent.get(v);
  return v;
}
