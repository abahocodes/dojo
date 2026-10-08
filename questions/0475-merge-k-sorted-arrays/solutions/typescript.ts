function mergeKSortedArrays(arrays: number[][]): number[] {
  // Binary min-heap of [value, arrayIndex, position], ordered by value.
  const heap: [number, number, number][] = [];
  for (let a = 0; a < arrays.length; a++) {
    if (arrays[a].length > 0) heap.push([arrays[a][0], a, 0]);
  }
  const siftDown = (i: number): void => {
    for (;;) {
      const l = 2 * i + 1;
      const r = l + 1;
      let m = i;
      if (l < heap.length && heap[l][0] < heap[m][0]) m = l;
      if (r < heap.length && heap[r][0] < heap[m][0]) m = r;
      if (m === i) return;
      [heap[m], heap[i]] = [heap[i], heap[m]];
      i = m;
    }
  };
  for (let i = (heap.length >> 1) - 1; i >= 0; i--) siftDown(i);

  const merged: number[] = [];
  while (heap.length > 0) {
    const [value, a, p] = heap[0];
    merged.push(value);
    if (p + 1 < arrays[a].length) {
      heap[0] = [arrays[a][p + 1], a, p + 1];
    } else {
      const last = heap.pop()!;
      if (heap.length === 0) break;
      heap[0] = last;
    }
    siftDown(0);
  }
  return merged;
}
