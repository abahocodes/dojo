function searchMatrixSorted(matrix, target) {
  const rows = matrix.length;
  let r = 0;
  let c = matrix[0].length - 1;
  while (r < rows && c >= 0) {
    const value = matrix[r][c];
    if (value === target) return true;
    if (value > target) c--;
    else r++;
  }
  return false;
}
