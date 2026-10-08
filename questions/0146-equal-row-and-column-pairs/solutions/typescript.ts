function equalPairs(grid: number[][]): number {
  const n = grid.length;
  const rows = new Map<string, number>();
  for (const row of grid) {
    const key = row.join(",");
    rows.set(key, (rows.get(key) ?? 0) + 1);
  }
  let pairs = 0;
  for (let c = 0; c < n; c++) {
    const col: number[] = [];
    for (let r = 0; r < n; r++) col.push(grid[r][c]);
    pairs += rows.get(col.join(",")) ?? 0;
  }
  return pairs;
}
