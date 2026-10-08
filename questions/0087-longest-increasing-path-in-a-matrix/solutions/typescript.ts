function longestIncreasingPath(matrix: number[][]): number {
  const rows = matrix.length;
  const cols = matrix[0].length;
  // cells by decreasing height, encoded as r * cols + c
  const order: number[] = [];
  for (let i = 0; i < rows * cols; i++) order.push(i);
  order.sort((a, b) => matrix[Math.floor(b / cols)][b % cols] - matrix[Math.floor(a / cols)][a % cols]);

  const best: number[][] = Array.from({ length: rows }, () => new Array<number>(cols).fill(1));
  const dirs: [number, number][] = [[1, 0], [-1, 0], [0, 1], [0, -1]];
  let answer = 1;
  for (const cell of order) {
    const r = Math.floor(cell / cols);
    const c = cell % cols;
    const height = matrix[r][c];
    let length = 1;
    for (const [dr, dc] of dirs) {
      const nr = r + dr;
      const nc = c + dc;
      if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && matrix[nr][nc] > height) {
        if (best[nr][nc] + 1 > length) length = best[nr][nc] + 1;
      }
    }
    best[r][c] = length;
    if (length > answer) answer = length;
  }
  return answer;
}
