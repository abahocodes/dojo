function findTarget(root: TreeNode | null, k: number): boolean {
  const values: number[] = [];
  const stack: TreeNode[] = [];
  let node = root;
  while (stack.length > 0 || node) {
    while (node) {
      stack.push(node);
      node = node.left;
    }
    const cur = stack.pop()!;
    values.push(cur.val);
    node = cur.right;
  }

  let i = 0;
  let j = values.length - 1;
  while (i < j) {
    const s = values[i] + values[j];
    if (s === k) return true;
    if (s < k) i++;
    else j--;
  }
  return false;
}
