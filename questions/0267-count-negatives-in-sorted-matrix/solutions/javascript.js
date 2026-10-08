function countNegatives(grid) {
  const n = grid[0].length;
  let row = grid.length - 1;
  let col = 0;
  let count = 0;
  while (row >= 0 && col < n) {
    if (grid[row][col] < 0) {
      count += n - col;
      row--;
    } else {
      col++;
    }
  }
  return count;
}
