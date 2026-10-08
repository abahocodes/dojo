function evaluateTree(root) {
  const value = new Map();
  const stack = [[root, false]];
  while (stack.length > 0) {
    const [node, childrenDone] = stack.pop();
    if (!node.left) {
      value.set(node, node.val === 1);
    } else if (childrenDone) {
      const left = value.get(node.left);
      const right = value.get(node.right);
      value.set(node, node.val === 2 ? left || right : left && right);
    } else {
      stack.push([node, true], [node.left, false], [node.right, false]);
    }
  }
  return value.get(root);
}
