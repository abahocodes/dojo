function lastStoneWeight(stones) {
  // Binary max-heap stored in an array.
  const heap = [];
  const push = (v) => {
    heap.push(v);
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
        let big = i;
        if (l < heap.length && heap[l] > heap[big]) big = l;
        if (r < heap.length && heap[r] > heap[big]) big = r;
        if (big === i) break;
        [heap[big], heap[i]] = [heap[i], heap[big]];
        i = big;
      }
    }
    return top;
  };

  for (const s of stones) push(s);
  while (heap.length > 1) {
    const y = pop();
    const x = pop();
    if (y !== x) push(y - x);
  }
  return heap.length ? heap[0] : 0;
}
