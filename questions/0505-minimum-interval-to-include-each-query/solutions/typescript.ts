function minInterval(intervals: number[][], queries: number[]): number[] {
  const sorted = intervals.slice().sort((a, b) => a[0] - b[0]);
  const order = queries.map((_, i) => i).sort((a, b) => queries[a] - queries[b]);
  const answer: number[] = new Array(queries.length).fill(-1);
  // Min-heap of [length, right end], ordered by length then right end.
  const heap: [number, number][] = [];
  const less = (a: [number, number], b: [number, number]): boolean => a[0] < b[0] || (a[0] === b[0] && a[1] < b[1]);
  const push = (item: [number, number]): void => {
    heap.push(item);
    let i = heap.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (!less(heap[i], heap[p])) break;
      [heap[i], heap[p]] = [heap[p], heap[i]];
      i = p;
    }
  };
  const pop = (): void => {
    const last = heap.pop()!;
    if (heap.length === 0) return;
    heap[0] = last;
    let i = 0;
    for (;;) {
      const l = 2 * i + 1;
      const r = l + 1;
      let small = i;
      if (l < heap.length && less(heap[l], heap[small])) small = l;
      if (r < heap.length && less(heap[r], heap[small])) small = r;
      if (small === i) return;
      [heap[i], heap[small]] = [heap[small], heap[i]];
      i = small;
    }
  };

  let i = 0;
  for (const qi of order) {
    const q = queries[qi];
    while (i < sorted.length && sorted[i][0] <= q) {
      const [left, right] = sorted[i];
      push([right - left + 1, right]);
      i++;
    }
    // Intervals that end before q can never contain a later (larger) query.
    while (heap.length > 0 && heap[0][1] < q) pop();
    if (heap.length > 0) answer[qi] = heap[0][0];
  }
  return answer;
}
