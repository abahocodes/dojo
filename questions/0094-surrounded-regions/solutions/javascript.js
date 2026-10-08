function captureRegions(board) {
  const rows = board.length;
  const cols = board[0].length;
  const safe = Array.from({ length: rows }, () => new Array(cols).fill(false));
  const stack = [];
  for (let r = 0; r < rows; r++) {
    for (let c = 0; c < cols; c++) {
      const onEdge = r === 0 || c === 0 || r === rows - 1 || c === cols - 1;
      if (onEdge && board[r][c] === "O") {
        safe[r][c] = true;
        stack.push([r, c]);
      }
    }
  }
  const dirs = [[1, 0], [-1, 0], [0, 1], [0, -1]];
  while (stack.length > 0) {
    const [r, c] = stack.pop();
    for (const [dr, dc] of dirs) {
      const nr = r + dr;
      const nc = c + dc;
      if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && board[nr][nc] === "O" && !safe[nr][nc]) {
        safe[nr][nc] = true;
        stack.push([nr, nc]);
      }
    }
  }
  return safe.map((row) => row.map((isSafe) => (isSafe ? "O" : "X")));
}
