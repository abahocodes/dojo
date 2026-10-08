function mincostToHireWorkers(quality, wage, k) {
  // Sort workers by wage/quality ratio, compared exactly by cross-multiplying.
  const order = quality.map((_, i) => i);
  order.sort((a, b) => wage[a] * quality[b] - wage[b] * quality[a]);

  // Max-heap of the qualities in the current group.
  const heap = [];
  const push = (x) => {
    heap.push(x);
    let i = heap.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (heap[p] >= heap[i]) break;
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
        if (l < heap.length && heap[l] > heap[m]) m = l;
        if (r < heap.length && heap[r] > heap[m]) m = r;
        if (m === i) break;
        [heap[m], heap[i]] = [heap[i], heap[m]];
        i = m;
      }
    }
    return top;
  };

  let totalQuality = 0;
  let best = Infinity;
  for (const i of order) {
    push(quality[i]);
    totalQuality += quality[i];
    if (heap.length > k) totalQuality -= pop(); // drop the largest quality
    if (heap.length === k) {
      // Worker i has the largest ratio so far and sets the pay rate.
      best = Math.min(best, (totalQuality * wage[i]) / quality[i]);
    }
  }
  return best;
}
