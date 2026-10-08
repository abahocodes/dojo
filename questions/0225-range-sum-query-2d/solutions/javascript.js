function regionSums(matrix, queries) {
  const m = matrix.length, n = matrix[0].length;
  // pre[i][j] = sum of matrix[0..i-1][0..j-1]
  const pre = Array.from({ length: m + 1 }, () => new Array(n + 1).fill(0));
  for (let i = 0; i < m; i++) {
    for (let j = 0; j < n; j++) {
      pre[i + 1][j + 1] = matrix[i][j] + pre[i][j + 1] + pre[i + 1][j] - pre[i][j];
    }
  }
  return queries.map(([r1, c1, r2, c2]) =>
    pre[r2 + 1][c2 + 1] - pre[r1][c2 + 1] - pre[r2 + 1][c1] + pre[r1][c1]);
}
