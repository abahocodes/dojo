function generateTrees(n: number): Array<TreeNode | null> {
  const memo = new Map<number, Array<TreeNode | null>>();
  function build(lo: number, hi: number): Array<TreeNode | null> {
    if (lo > hi) return [null];
    const key = lo * 100 + hi;
    const cached = memo.get(key);
    if (cached !== undefined) return cached;
    const trees: Array<TreeNode | null> = [];
    for (let v = lo; v <= hi; v++) {
      for (const left of build(lo, v - 1)) {
        for (const right of build(v + 1, hi)) {
          trees.push(new TreeNode(v, left, right));
        }
      }
    }
    memo.set(key, trees);
    return trees;
  }
  return build(1, n);
}
