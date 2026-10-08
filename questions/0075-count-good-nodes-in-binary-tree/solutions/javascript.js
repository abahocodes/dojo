function goodNodes(root) {
  let count = 0;
  const stack = [[root, root.val]];
  while (stack.length > 0) {
    const [node, seen] = stack.pop();
    let best = seen;
    if (node.val >= best) {
      count += 1;
      best = node.val;
    }
    if (node.left) stack.push([node.left, best]);
    if (node.right) stack.push([node.right, best]);
  }
  return count;
}
