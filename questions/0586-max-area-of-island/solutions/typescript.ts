function maxAreaOfIsland(grid: number[][]): number {
  const rows = grid.length;
  const cols = rows ? grid[0].length : 0;
  const seen: boolean[][] = Array.from({ length: rows }, () => new Array<boolean>(cols).fill(false));
  const dirs = [[1, 0], [-1, 0], [0, 1], [0, -1]];
  let best = 0;
  for (let r = 0; r < rows; r++) {
    for (let c = 0; c < cols; c++) {
      if (grid[r][c] !== 1 || seen[r][c]) continue;
      // Iterative DFS: count every land cell reachable from (r, c).
      seen[r][c] = true;
      const stack: [number, number][] = [[r, c]];
      let area = 0;
      while (stack.length) {
        const [y, x] = stack.pop()!;
        area++;
        for (const [dy, dx] of dirs) {
          const ny = y + dy;
          const nx = x + dx;
          if (ny >= 0 && ny < rows && nx >= 0 && nx < cols && grid[ny][nx] === 1 && !seen[ny][nx]) {
            seen[ny][nx] = true;
            stack.push([ny, nx]);
          }
        }
      }
      best = Math.max(best, area);
    }
  }
  return best;
}
