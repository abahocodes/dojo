function kthSmallestMatrix(matrix, k) {
  const n = matrix.length;
  const countAtMost = (v) => {
    // Staircase walk from the bottom-left corner.
    let count = 0;
    let row = n - 1;
    let col = 0;
    while (row >= 0 && col < n) {
      if (matrix[row][col] <= v) {
        count += row + 1;
        col++;
      } else {
        row--;
      }
    }
    return count;
  };
  let lo = matrix[0][0];
  let hi = matrix[n - 1][n - 1];
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    if (countAtMost(mid) >= k) hi = mid;
    else lo = mid + 1;
  }
  return lo;
}
