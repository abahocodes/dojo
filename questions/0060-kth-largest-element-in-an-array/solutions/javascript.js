function findKthLargest(nums, k) {
  // min-heap holding the k largest values seen so far; heap[0] is the smallest of them
  const heap = [];

  function siftUp(i) {
    while (i > 0) {
      const parent = (i - 1) >> 1;
      if (heap[parent] <= heap[i]) break;
      [heap[parent], heap[i]] = [heap[i], heap[parent]];
      i = parent;
    }
  }

  function siftDown(i) {
    for (;;) {
      const left = 2 * i + 1;
      const right = left + 1;
      let smallest = i;
      if (left < heap.length && heap[left] < heap[smallest]) smallest = left;
      if (right < heap.length && heap[right] < heap[smallest]) smallest = right;
      if (smallest === i) return;
      [heap[smallest], heap[i]] = [heap[i], heap[smallest]];
      i = smallest;
    }
  }

  for (const x of nums) {
    if (heap.length < k) {
      heap.push(x);
      siftUp(heap.length - 1);
    } else if (x > heap[0]) {
      heap[0] = x;
      siftDown(0);
    }
  }
  return heap[0];
}
