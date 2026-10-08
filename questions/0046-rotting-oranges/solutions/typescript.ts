function orangesRotting(grid: number[][]): number {
  const rows = grid.length;
  const cols = grid[0].length;
  const state = grid.map((row) => row.slice()); // don't mutate the caller's grid
  let frontier: [number, number][] = [];
  let fresh = 0;
  for (let r = 0; r < rows; r++) {
    for (let c = 0; c < cols; c++) {
      if (state[r][c] === 2) frontier.push([r, c]);
      else if (state[r][c] === 1) fresh++;
    }
  }

  const dirs = [[1, 0], [-1, 0], [0, 1], [0, -1]];
  let minutes = 0;
  while (frontier.length > 0 && fresh > 0) {
    minutes++;
    const next: [number, number][] = [];
    for (const [r, c] of frontier) {
      for (const [dr, dc] of dirs) {
        const nr = r + dr;
        const nc = c + dc;
        if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && state[nr][nc] === 1) {
          state[nr][nc] = 2;
          fresh--;
          next.push([nr, nc]);
        }
      }
    }
    frontier = next;
  }
  return fresh === 0 ? minutes : -1;
}
