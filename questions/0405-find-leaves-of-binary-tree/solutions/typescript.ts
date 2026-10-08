function findLeaves(root: TreeNode | null): number[][] {
  const result: number[][] = [];
  const height = (node: TreeNode | null): number => {
    if (node === null) return -1;
    const h = Math.max(height(node.left), height(node.right)) + 1;
    if (h === result.length) result.push([]);
    result[h].push(node.val);
    return h;
  };
  height(root);
  return result;
}
