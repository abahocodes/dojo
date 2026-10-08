function hasPathSum(root, targetSum) {
  if (root === null) return false;
  const stack = [[root, targetSum - root.val]];
  while (stack.length > 0) {
    const [node, remaining] = stack.pop();
    if (node.left === null && node.right === null) {
      if (remaining === 0) return true;
      continue;
    }
    if (node.left) stack.push([node.left, remaining - node.left.val]);
    if (node.right) stack.push([node.right, remaining - node.right.val]);
  }
  return false;
}
