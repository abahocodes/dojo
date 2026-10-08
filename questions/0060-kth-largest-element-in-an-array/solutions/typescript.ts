function findKthLargest(nums: number[], k: number): number {
  // Min-heap holding the k largest values seen so far; heap[0] is the smallest of them.
  const heap = nums.slice(0, k);
  const siftDown = (start: number): void => {
    let i = start;
    for (;;) {
      const l = 2 * i + 1;
      const r = l + 1;
      let smallest = i;
      if (l < k && heap[l] < heap[smallest]) smallest = l;
      if (r < k && heap[r] < heap[smallest]) smallest = r;
      if (smallest === i) return;
      [heap[smallest], heap[i]] = [heap[i], heap[smallest]];
      i = smallest;
    }
  };
  for (let i = (k >> 1) - 1; i >= 0; i--) siftDown(i);
  for (let i = k; i < nums.length; i++) {
    if (nums[i] > heap[0]) {
      heap[0] = nums[i];
      siftDown(0);
    }
  }
  return heap[0];
}
