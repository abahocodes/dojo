function maximalSquare(matrix: string[][]): number {
  const cols = matrix[0].length;
  const side = new Array<number>(cols + 1).fill(0);
  let best = 0;
  for (const row of matrix) {
    let prevDiag = 0;
    for (let c = 1; c <= cols; c++) {
      const above = side[c];
      if (row[c - 1] === "1") {
        side[c] = 1 + Math.min(above, side[c - 1], prevDiag);
        if (side[c] > best) best = side[c];
      } else {
        side[c] = 0;
      }
      prevDiag = above;
    }
  }
  return best * best;
}
