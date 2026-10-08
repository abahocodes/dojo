function kSmallestPairs(nums1: number[], nums2: number[], k: number): number[][] {
  // Binary min-heap of [sum, i, j], ordered by sum, then i, then j.
  type Entry = [number, number, number];
  const less = (a: Entry, b: Entry): boolean =>
    a[0] !== b[0] ? a[0] < b[0] : a[1] !== b[1] ? a[1] < b[1] : a[2] < b[2];
  const heap: Entry[] = [];
  const push = (item: Entry): void => {
    heap.push(item);
    let i = heap.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (!less(heap[i], heap[p])) break;
      [heap[p], heap[i]] = [heap[i], heap[p]];
      i = p;
    }
  };
  const pop = (): Entry => {
    const top = heap[0];
    const last = heap.pop()!;
    if (heap.length > 0) {
      heap[0] = last;
      let i = 0;
      for (;;) {
        const l = 2 * i + 1;
        const r = l + 1;
        let m = i;
        if (l < heap.length && less(heap[l], heap[m])) m = l;
        if (r < heap.length && less(heap[r], heap[m])) m = r;
        if (m === i) break;
        [heap[m], heap[i]] = [heap[i], heap[m]];
        i = m;
      }
    }
    return top;
  };

  for (let i = 0; i < Math.min(k, nums1.length); i++) push([nums1[i] + nums2[0], i, 0]);
  const result: number[][] = [];
  while (heap.length > 0 && result.length < k) {
    const [, i, j] = pop();
    result.push([nums1[i], nums2[j]]);
    if (j + 1 < nums2.length) push([nums1[i] + nums2[j + 1], i, j + 1]);
  }
  return result;
}
