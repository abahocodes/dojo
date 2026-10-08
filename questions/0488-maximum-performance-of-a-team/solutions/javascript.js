function maxPerformance(n, speed, efficiency, k) {
  const order = Array.from({ length: n }, (_, i) => i);
  order.sort((a, b) => efficiency[b] - efficiency[a]);

  // Min-heap of the speeds in the current team.
  const heap = [];
  const push = (x) => {
    heap.push(x);
    let i = heap.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (heap[p] <= heap[i]) break;
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
        let s = i;
        if (l < heap.length && heap[l] < heap[s]) s = l;
        if (r < heap.length && heap[r] < heap[s]) s = r;
        if (s === i) break;
        [heap[s], heap[i]] = [heap[i], heap[s]];
        i = s;
      }
    }
    return top;
  };

  let totalSpeed = 0; // at most 10^10: exact as a double
  let best = 0n;      // the product reaches 10^18, so compare as BigInt
  for (const i of order) {
    // efficiency[i] is the smallest efficiency so far: it is the team minimum.
    push(speed[i]);
    totalSpeed += speed[i];
    if (heap.length > k) totalSpeed -= pop();
    const performance = BigInt(totalSpeed) * BigInt(efficiency[i]);
    if (performance > best) best = performance;
  }
  return Number(best % 1000000007n);
}
