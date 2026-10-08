function matrixBlockSum(mat: number[][], k: number): number[][] {
  const m = mat.length, n = mat[0].length;
  const pre: number[][] = Array.from({ length: m + 1 }, () => new Array<number>(n + 1).fill(0));
  for (let i = 0; i < m; i++) {
    for (let j = 0; j < n; j++) {
      pre[i + 1][j + 1] = mat[i][j] + pre[i][j + 1] + pre[i + 1][j] - pre[i][j];
    }
  }
  const result: number[][] = [];
  for (let i = 0; i < m; i++) {
    const r1 = Math.max(0, i - k), r2 = Math.min(m, i + k + 1);
    const row: number[] = new Array(n);
    for (let j = 0; j < n; j++) {
      const c1 = Math.max(0, j - k), c2 = Math.min(n, j + k + 1);
      row[j] = pre[r2][c2] - pre[r1][c2] - pre[r2][c1] + pre[r1][c1];
    }
    result.push(row);
  }
  return result;
}
