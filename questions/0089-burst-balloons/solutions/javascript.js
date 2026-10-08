function maxCoins(nums) {
  const v = [1, ...nums, 1];
  const n = v.length;
  const best = Array.from({ length: n }, () => new Array(n).fill(0));
  for (let length = 2; length < n; length++) {
    for (let i = 0; i + length < n; i++) {
      const j = i + length;
      const edge = v[i] * v[j];
      let top = 0;
      for (let k = i + 1; k < j; k++) {
        const total = best[i][k] + edge * v[k] + best[k][j];
        if (total > top) top = total;
      }
      best[i][j] = top;
    }
  }
  return best[0][n - 1];
}
