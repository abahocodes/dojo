function numIslands(grid: string[][]): number {
  if (grid.length === 0) return 0;
  const rows = grid.length;
  const cols = grid[0].length;
  const seen = Array.from({ length: rows }, () => new Array<boolean>(cols).fill(false));
  let count = 0;
  for (let r = 0; r < rows; r++) {
    for (let c = 0; c < cols; c++) {
      if (grid[r][c] !== "1" || seen[r][c]) continue;
      count++;
      seen[r][c] = true;
      const stack: [number, number][] = [[r, c]];
      while (stack.length > 0) {
        const [i, j] = stack.pop()!;
        const neighbors: [number, number][] = [[i + 1, j], [i - 1, j], [i, j + 1], [i, j - 1]];
        for (const [ni, nj] of neighbors) {
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
