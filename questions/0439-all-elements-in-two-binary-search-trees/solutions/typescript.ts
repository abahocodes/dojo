function getAllElements(root1: TreeNode | null, root2: TreeNode | null): number[] {
  function inorder(root: TreeNode | null): number[] {
    const values: number[] = [];
    const stack: TreeNode[] = [];
    let node: TreeNode | null = root;
    while (stack.length > 0 || node !== null) {
      while (node !== null) {
        stack.push(node);
        node = node.left;
      }
      const top = stack.pop()!;
      values.push(top.val);
      node = top.right;
    }
    return values;
  }
  const a = inorder(root1);
  const b = inorder(root2);
  const merged: number[] = [];
  let i = 0;
  let j = 0;
  while (i < a.length && j < b.length) {
    if (a[i] <= b[j]) merged.push(a[i++]);
    else merged.push(b[j++]);
  }
  while (i < a.length) merged.push(a[i++]);
  while (j < b.length) merged.push(b[j++]);
  return merged;
}
