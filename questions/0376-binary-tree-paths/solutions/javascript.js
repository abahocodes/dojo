function binaryTreePaths(root) {
  const paths = [];
  const stack = [[root, String(root.val)]];
  while (stack.length > 0) {
    const [node, path] = stack.pop();
    if (node.left === null && node.right === null) {
      paths.push(path);
      continue;
    }
    if (node.right) stack.push([node.right, path + "->" + node.right.val]);
    if (node.left) stack.push([node.left, path + "->" + node.left.val]);
  }
  return paths;
}
