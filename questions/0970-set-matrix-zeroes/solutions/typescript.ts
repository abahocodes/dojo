function setZeroes(matrix: number[][]): number[][] {
  const rows = matrix.length;
  const cols = matrix[0].length;
  // Row 0 and column 0 double as markers; remember separately whether
  // they themselves need clearing.
  let firstRowZero = false;
  let firstColZero = false;
  for (let c = 0; c < cols; c++) if (matrix[0][c] === 0) firstRowZero = true;
  for (let r = 0; r < rows; r++) if (matrix[r][0] === 0) firstColZero = true;

  for (let r = 1; r < rows; r++) {
    for (let c = 1; c < cols; c++) {
      if (matrix[r][c] === 0) {
        matrix[r][0] = 0;
        matrix[0][c] = 0;
      }
    }
  }

  for (let r = 1; r < rows; r++) {
    for (let c = 1; c < cols; c++) {
      if (matrix[r][0] === 0 || matrix[0][c] === 0) matrix[r][c] = 0;
    }
  }

  if (firstRowZero) for (let c = 0; c < cols; c++) matrix[0][c] = 0;
  if (firstColZero) for (let r = 0; r < rows; r++) matrix[r][0] = 0;
  return matrix;
}
