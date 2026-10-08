function connectSticks(sticks: number[]): number {
  // Binary min-heap of stick lengths.
  const heap: number[] = sticks.slice();
  const siftDown = (i: number): void => {
    for (;;) {
      const l = 2 * i + 1;
      const r = l + 1;
      let m = i;
      if (l < heap.length && heap[l] < heap[m]) m = l;
      if (r < heap.length && heap[r] < heap[m]) m = r;
      if (m === i) return;
      [heap[m], heap[i]] = [heap[i], heap[m]];
      i = m;
    }
  };
  const pop = (): number => {
    const top = heap[0];
    const last = heap.pop()!;
    if (heap.length > 0) {
      heap[0] = last;
      siftDown(0);
    }
    return top;
  };
  for (let i = (heap.length >> 1) - 1; i >= 0; i--) siftDown(i);

  let total = 0;
  while (heap.length > 1) {
    const joined = pop() + heap[0];
    total += joined;
    heap[0] = joined; // replace the second-smallest with the joined stick
    siftDown(0);
  }
  return total;
}
