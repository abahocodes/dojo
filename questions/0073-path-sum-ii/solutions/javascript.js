function pathSum(root, targetSum) {
  const result = [];
  if (root === null) return result;
  const path = [];
  let total = 0;
  const stack = [[root, false]];
  while (stack.length > 0) {
    const [node, leaving] = stack.pop();
    if (leaving) {
      path.pop();
      total -= node.val;
      continue;
    }
    path.push(node.val);
    total += node.val;
    stack.push([node, true]);
    if (node.left === null && node.right === null) {
      if (total === targetSum) result.push(path.slice());
    } else {
      if (node.right) stack.push([node.right, false]);
      if (node.left) stack.push([node.left, false]);
    }
  }
  return result;
}
