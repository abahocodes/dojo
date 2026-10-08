function numSubmatrixSumTarget(matrix: number[][], target: number): number {
  const rows = matrix.length;
  const cols = matrix[0].length;
  let count = 0;
  for (let top = 0; top < rows; top++) {
    const col: number[] = new Array(cols).fill(0);
    for (let bottom = top; bottom < rows; bottom++) {
      const row = matrix[bottom];
      for (let c = 0; c < cols; c++) col[c] += row[c];
      const seen = new Map<number, number>([[0, 1]]);
      let s = 0;
      for (const v of col) {
        s += v;
        count += seen.get(s - target) || 0;
        seen.set(s, (seen.get(s) || 0) + 1);
      }
    }
  }
  return count;
}
