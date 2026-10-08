function kthLargestStream(k, nums, adds) {
  // Min-heap holding the k largest values seen so far; its top is the answer.
  const heap = [];
  const siftDown = (i) => {
    for (;;) {
      const l = 2 * i + 1;
      const r = l + 1;
      let small = i;
      if (l < heap.length && heap[l] < heap[small]) small = l;
      if (r < heap.length && heap[r] < heap[small]) small = r;
      if (small === i) return;
      [heap[small], heap[i]] = [heap[i], heap[small]];
      i = small;
    }
  };
  const add = (v) => {
    if (heap.length < k) {
      heap.push(v);
      let i = heap.length - 1;
      while (i > 0) {
        const p = (i - 1) >> 1;
        if (heap[p] <= heap[i]) break;
        [heap[p], heap[i]] = [heap[i], heap[p]];
        i = p;
      }
    } else if (v > heap[0]) {
      heap[0] = v;
      siftDown(0);
    }
  };

  for (const v of nums) add(v);
  const result = [];
  for (const v of adds) {
    add(v);
    result.push(heap[0]);
  }
  return result;
}
