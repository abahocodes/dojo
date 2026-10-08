function pacificAtlantic(heights: number[][]): number[][] {
  const rows = heights.length;
  const cols = heights[0].length;
  const dirs = [[1, 0], [-1, 0], [0, 1], [0, -1]];

  // Walk uphill from the ocean: water can flow from each reached cell
  // down to the ocean. Iterative DFS keeps large grids off the call stack.
  const reachable = (starts: [number, number][]): boolean[][] => {
    const seen = Array.from({ length: rows }, () => new Array<boolean>(cols).fill(false));
    const stack: [number, number][] = [];
    for (const [r, c] of starts) {
      if (!seen[r][c]) {
        seen[r][c] = true;
        stack.push([r, c]);
      }
    }
    while (stack.length > 0) {
      const [r, c] = stack.pop()!;
      for (const [dr, dc] of dirs) {
        const nr = r + dr;
        const nc = c + dc;
        if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && !seen[nr][nc]
            && heights[nr][nc] >= heights[r][c]) {
          seen[nr][nc] = true;
          stack.push([nr, nc]);
        }
      }
    }
    return seen;
  };

  const pacificStarts: [number, number][] = [];
  const atlanticStarts: [number, number][] = [];
  for (let c = 0; c < cols; c++) {
    pacificStarts.push([0, c]);
    atlanticStarts.push([rows - 1, c]);
  }
  for (let r = 0; r < rows; r++) {
    pacificStarts.push([r, 0]);
    atlanticStarts.push([r, cols - 1]);
  }
  const pacific = reachable(pacificStarts);
  const atlantic = reachable(atlanticStarts);
  const result: number[][] = [];
  for (let r = 0; r < rows; r++) {
    for (let c = 0; c < cols; c++) {
      if (pacific[r][c] && atlantic[r][c]) result.push([r, c]);
    }
  }
  return result;
}
