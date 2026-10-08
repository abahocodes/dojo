function trapRainWater2d(heightMap: number[][]): number {
  const m = heightMap.length;
  const n = heightMap[0].length;
  const visited: boolean[][] = Array.from({ length: m }, () => new Array<boolean>(n).fill(false));

  // Min-heap of [level, row, col] on the frontier, ordered by level.
  const heap: [number, number, number][] = [];
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
        let s = i;
        if (l < heap.length && heap[l][0] < heap[s][0]) s = l;
        if (r < heap.length && heap[r][0] < heap[s][0]) s = r;
        if (s === i) break;
        [heap[s], heap[i]] = [heap[i], heap[s]];
        i = s;
      }
    }
    return top;
  };

  for (let r = 0; r < m; r++) {
    for (let c = 0; c < n; c++) {
      if (r === 0 || c === 0 || r === m - 1 || c === n - 1) {
        push([heightMap[r][c], r, c]);
        visited[r][c] = true;
      }
    }
  }

  const dirs: [number, number][] = [[1, 0], [-1, 0], [0, 1], [0, -1]];
  let total = 0;
  while (heap.length > 0) {
    const [level, r, c] = pop();
    for (const [dr, dc] of dirs) {
      const nr = r + dr;
      const nc = c + dc;
      if (nr < 0 || nc < 0 || nr >= m || nc >= n || visited[nr][nc]) continue;
      visited[nr][nc] = true;
      const h = heightMap[nr][nc];
      if (h < level) total += level - h;
      push([Math.max(h, level), nr, nc]);
    }
  }
  return total;
}
