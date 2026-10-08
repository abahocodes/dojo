function searchMatrix(matrix: number[][], target: number): boolean {
  const cols = matrix[0].length;
  let lo = 0;
  let hi = matrix.length * cols - 1;
  while (lo <= hi) {
    const mid = (lo + hi) >> 1;
    const value = matrix[Math.floor(mid / cols)][mid % cols];
    if (value === target) return true;
    if (value < target) lo = mid + 1;
    else hi = mid - 1;
  }
  return false;
}
