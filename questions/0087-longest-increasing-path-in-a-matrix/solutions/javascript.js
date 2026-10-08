function longestIncreasingPath(matrix) {
  const rows = matrix.length;
  const cols = matrix[0].length;
  const order = [];
  for (let r = 0; r < rows; r++) {
    for (let c = 0; c < cols; c++) order.push(r * cols + c);
  }
  const height = (i) => matrix[Math.floor(i / cols)][i % cols];
  order.sort((a, b) => height(b) - height(a));
  const best = new Int32Array(rows * cols);
  const dirs = [[1, 0], [-1, 0], [0, 1], [0, -1]];
  let answer = 1;
  for (const i of order) {
    const r = Math.floor(i / cols);
    const c = i % cols;
    let length = 1;
    for (const [dr, dc] of dirs) {
      const nr = r + dr;
      const nc = c + dc;
      if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && matrix[nr][nc] > matrix[r][c]) {
        length = Math.max(length, best[nr * cols + nc] + 1);
      }
    }
    best[i] = length;
    answer = Math.max(answer, length);
  }
  return answer;
}
