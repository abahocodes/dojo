function diagonalSort(mat) {
  const m = mat.length, n = mat[0].length;
  const res = mat.map((row) => row.slice());
  const sortFrom = (si, sj) => {
    const length = Math.min(m - si, n - sj);
    const values = [];
    for (let k = 0; k < length; k++) values.push(res[si + k][sj + k]);
    values.sort((a, b) => a - b);
    for (let k = 0; k < length; k++) res[si + k][sj + k] = values[k];
  };
  for (let i = 0; i < m; i++) sortFrom(i, 0);
  for (let j = 1; j < n; j++) sortFrom(0, j);
  return res;
}
