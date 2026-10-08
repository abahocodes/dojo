function delNodes(root, toDelete) {
  const doomed = new Set(toDelete);
  const forest = [];
  // Each entry: [node, is it the top of a tree once its parent is gone?]
  const stack = [[root, true]];
  while (stack.length > 0) {
    const [node, isRoot] = stack.pop();
    const deleted = doomed.has(node.val);
    if (isRoot && !deleted) forest.push(node);
    if (node.left !== null) stack.push([node.left, deleted]);
    if (node.right !== null) stack.push([node.right, deleted]);
    if (node.left !== null && doomed.has(node.left.val)) node.left = null;
    if (node.right !== null && doomed.has(node.right.val)) node.right = null;
  }
  return forest;
}
