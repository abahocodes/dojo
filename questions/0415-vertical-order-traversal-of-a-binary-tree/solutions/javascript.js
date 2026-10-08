function verticalTraversal(root) {
  const entries = []; // [col, row, val]
  const stack = [[root, 0, 0]];
  while (stack.length > 0) {
    const [node, row, col] = stack.pop();
    entries.push([col, row, node.val]);
    if (node.left) stack.push([node.left, row + 1, col - 1]);
    if (node.right) stack.push([node.right, row + 1, col + 1]);
  }
  entries.sort((a, b) => a[0] - b[0] || a[1] - b[1] || a[2] - b[2]);
  const result = [];
  let prevCol = null;
  for (const [col, , val] of entries) {
    if (col !== prevCol) {
      result.push([]);
      prevCol = col;
    }
    result[result.length - 1].push(val);
  }
  return result;
}
