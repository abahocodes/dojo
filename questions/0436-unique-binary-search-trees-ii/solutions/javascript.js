function generateTrees(n) {
  const memo = new Map();
  function build(lo, hi) {
    if (lo > hi) return [null];
    const key = lo * 100 + hi;
    if (memo.has(key)) return memo.get(key);
    const trees = [];
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
