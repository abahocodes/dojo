function numIslands(grid) {
  const rows = grid.length;
  if (rows === 0) return 0;
  const cols = grid[0].length;
  const seen = Array.from({ length: rows }, () => new Array(cols).fill(false));
  const dirs = [[1, 0], [-1, 0], [0, 1], [0, -1]];
  let count = 0;

  for (let r = 0; r < rows; r++) {
    for (let c = 0; c < cols; c++) {
      if (grid[r][c] !== "1" || seen[r][c]) continue;
      count++;
      seen[r][c] = true;
      const stack = [[r, c]];
      while (stack.length > 0) {
        const [i, j] = stack.pop();
        for (const [di, dj] of dirs) {
          const ni = i + di;
          const nj = j + dj;
          if (ni >= 0 && ni < rows && nj >= 0 && nj < cols && grid[ni][nj] === "1" && !seen[ni][nj]) {
            seen[ni][nj] = true;
            stack.push([ni, nj]);
          }
        }
      }
    }
  }
  return count;
}
