function rightSideView(root) {
  if (root === null) return [];
  const view = [];
  let level = [root];
  while (level.length > 0) {
    view.push(level[level.length - 1].val);
    const next = [];
    for (const node of level) {
      if (node.left) next.push(node.left);
      if (node.right) next.push(node.right);
    }
    level = next;
  }
  return view;
}
