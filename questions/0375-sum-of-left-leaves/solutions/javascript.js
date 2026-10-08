function sumOfLeftLeaves(root) {
  let total = 0;
  const stack = [[root, false]];
  while (stack.length > 0) {
    const [node, isLeft] = stack.pop();
    if (node.left === null && node.right === null) {
      if (isLeft) total += node.val;
      continue;
    }
    if (node.left) stack.push([node.left, true]);
    if (node.right) stack.push([node.right, false]);
  }
  return total;
}
