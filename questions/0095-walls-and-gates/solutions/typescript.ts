const EMPTY = 2147483647;

function wallsAndGates(rooms: number[][]): number[][] {
  const rows = rooms.length;
  const cols = rooms[0].length;
  const dist = rooms.map((row) => row.slice());
  const queue: number[] = [];
  for (let r = 0; r < rows; r++) {
    for (let c = 0; c < cols; c++) {
      if (dist[r][c] === 0) queue.push(r * cols + c);
    }
  }
  const dirs = [[1, 0], [-1, 0], [0, 1], [0, -1]];
  for (let head = 0; head < queue.length; head++) {
    const r = Math.floor(queue[head] / cols);
    const c = queue[head] % cols;
    const d = dist[r][c] + 1;
    for (const [dr, dc] of dirs) {
      const nr = r + dr;
      const nc = c + dc;
      if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && dist[nr][nc] === EMPTY) {
        dist[nr][nc] = d;
        queue.push(nr * cols + nc);
      }
    }
  }
  return dist;
}
