function evaluateTree(root: TreeNode | null): boolean {
  if (root === null) return false;
  const value = new Map<TreeNode, boolean>();
  const stack: [TreeNode, boolean][] = [[root, false]];
  while (stack.length > 0) {
    const [node, childrenDone] = stack.pop()!;
    if (!node.left || !node.right) {
      value.set(node, node.val === 1);
    } else if (childrenDone) {
      const left = value.get(node.left)!;
      const right = value.get(node.right)!;
      value.set(node, node.val === 2 ? left || right : left && right);
    } else {
      stack.push([node, true], [node.left, false], [node.right, false]);
    }
  }
  return value.get(root)!;
}
