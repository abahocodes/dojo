function networkDelayTime(times, n, k) {
  const graph = Array.from({ length: n + 1 }, () => []);
  for (const [u, v, w] of times) graph[u].push([v, w]);

  // Minimal binary min-heap of [time, node] pairs.
  const heap = [];
  const push = (item) => {
    heap.push(item);
    let i = heap.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (heap[p][0] <= heap[i][0]) break;
      [heap[p], heap[i]] = [heap[i], heap[p]];
      i = p;
    }
  };
  const pop = () => {
    const top = heap[0];
    const last = heap.pop();
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

  const dist = new Array(n + 1).fill(-1);
  push([0, k]);
  while (heap.length > 0) {
    const [d, node] = pop();
    if (dist[node] !== -1) continue; // stale entry: already settled
    dist[node] = d;
    for (const [next, w] of graph[node]) {
      if (dist[next] === -1) push([d + w, next]);
    }
  }

  let worst = 0;
  for (let i = 1; i <= n; i++) {
    if (dist[i] === -1) return -1;
    worst = Math.max(worst, dist[i]);
  }
  return worst;
}
