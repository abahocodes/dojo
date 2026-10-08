function closestKValues(root: TreeNode | null, target: number, k: number): number[] {
  // pred: path to values <= target (top = largest); succ: values > target (top = smallest).
  const pred: TreeNode[] = [];
  const succ: TreeNode[] = [];
  let node: TreeNode | null = root;
  while (node !== null) {
    if (node.val <= target) {
      pred.push(node);
      node = node.right;
    } else {
      succ.push(node);
      node = node.left;
    }
  }
  const nextSmaller = (): number => {
    const top = pred.pop()!;
    for (let cur = top.left; cur !== null; cur = cur.right) pred.push(cur);
    return top.val;
  };
  const nextLarger = (): number => {
    const top = succ.pop()!;
    for (let cur = top.right; cur !== null; cur = cur.left) succ.push(cur);
    return top.val;
  };
  const result: number[] = [];
  for (let i = 0; i < k; i++) {
    if (succ.length === 0 ||
        (pred.length > 0 && target - pred[pred.length - 1].val <= succ[succ.length - 1].val - target)) {
      result.push(nextSmaller());
    } else {
      result.push(nextLarger());
    }
  }
  return result;
}
