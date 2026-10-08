function swimInWater(grid: number[][]): number {
  const n = grid.length;
  const seen: boolean[][] = Array.from({ length: n }, () => new Array(n).fill(false));
  seen[0][0] = true;
  // Binary min-heap of [height, row, col].
  const heap: [number, number, number][] = [[grid[0][0], 0, 0]];
  const push = (item: [number, number, number]): void => {
    heap.push(item);
    let i = heap.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (heap[p][0] <= heap[i][0]) break;
      [heap[p], heap[i]] = [heap[i], heap[p]];
      i = p;
    }
  };
  const pop = (): [number, number, number] => {
    const top = heap[0];
    const last = heap.pop()!;
    if (heap.length > 0) {
      heap[0] = last;
      let i = 0;
      for (;;) {
        const l = 2 * i + 1;
        const r = l + 1;
        let m = i;
        if (l < heap.length && heap[l][0] < heap[m][0]) m = l;
        if (r < heap.length && heap[r][0] < heap[m][0]) m = r;
        if (m === i) break;
        [heap[m], heap[i]] = [heap[i], heap[m]];
        i = m;
      }
    }
    return top;
  };
  const dirs = [[1, 0], [-1, 0], [0, 1], [0, -1]];
  let level = 0;
  while (heap.length > 0) {
    const [h, r, c] = pop();
    level = Math.max(level, h);
    if (r === n - 1 && c === n - 1) return level;
    for (const [dr, dc] of dirs) {
      const nr = r + dr;
      const nc = c + dc;
      if (nr >= 0 && nr < n && nc >= 0 && nc < n && !seen[nr][nc]) {
        seen[nr][nc] = true;
        push([grid[nr][nc], nr, nc]);
      }
    }
  }
  return level;
}
