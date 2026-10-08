function networkDelayTime(times: number[][], n: number, k: number): number {
  const graph: [number, number][][] = Array.from({ length: n + 1 }, () => []);
  for (const [u, v, w] of times) graph[u].push([v, w]);

  // Binary min-heap of [time, node] pairs, ordered by time.
  const heap: [number, number][] = [[0, k]];
  const push = (item: [number, number]): void => {
    heap.push(item);
    let i = heap.length - 1;
    while (i > 0) {
      const parent = (i - 1) >> 1;
      if (heap[parent][0] <= heap[i][0]) break;
      [heap[parent], heap[i]] = [heap[i], heap[parent]];
      i = parent;
    }
  };
  const pop = (): [number, number] => {
    const top = heap[0];
    const last = heap.pop()!;
    if (heap.length > 0) {
      heap[0] = last;
      let i = 0;
      for (;;) {
        const l = 2 * i + 1;
        const r = l + 1;
        let smallest = i;
        if (l < heap.length && heap[l][0] < heap[smallest][0]) smallest = l;
        if (r < heap.length && heap[r][0] < heap[smallest][0]) smallest = r;
        if (smallest === i) break;
        [heap[smallest], heap[i]] = [heap[i], heap[smallest]];
        i = smallest;
      }
    }
    return top;
  };

  const dist: (number | null)[] = new Array(n + 1).fill(null);
  while (heap.length > 0) {
    const [d, node] = pop();
    if (dist[node] !== null) continue; // stale entry: already settled with a smaller time
    dist[node] = d;
    for (const [next, w] of graph[node]) {
      if (dist[next] === null) push([d + w, next]);
    }
  }

  let best = 0;
  for (let i = 1; i <= n; i++) {
    const d = dist[i];
    if (d === null) return -1;
    best = Math.max(best, d);
  }
  return best;
}
