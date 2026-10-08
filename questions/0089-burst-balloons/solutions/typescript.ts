function maxCoins(nums: number[]): number {
  const v = [1, ...nums, 1];
  const n = v.length;
  // best[i][j] is the most coins from bursting everything strictly between i and j
  const best: number[][] = Array.from({ length: n }, () => new Array<number>(n).fill(0));
  for (let length = 2; length < n; length++) {
    for (let i = 0; i + length < n; i++) {
      const j = i + length;
      const edge = v[i] * v[j];
      const rowI = best[i];
      let top = 0;
      for (let k = i + 1; k < j; k++) {
        const total = rowI[k] + edge * v[k] + best[k][j];
        if (total > top) top = total;
      }
      rowI[j] = top;
    }
  }
  return best[0][n - 1];
}
