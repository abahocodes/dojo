function pickGifts(gifts, k) {
  // Binary max-heap of pile sizes.
  const heap = [];
  const siftDown = (i) => {
    for (;;) {
      const l = 2 * i + 1;
      const r = l + 1;
      let m = i;
      if (l < heap.length && heap[l] > heap[m]) m = l;
      if (r < heap.length && heap[r] > heap[m]) m = r;
      if (m === i) return;
      [heap[m], heap[i]] = [heap[i], heap[m]];
      i = m;
    }
  };
  for (const g of gifts) heap.push(g);
  for (let i = (heap.length >> 1) - 1; i >= 0; i--) siftDown(i);

  for (let s = 0; s < k; s++) {
    heap[0] = Math.floor(Math.sqrt(heap[0]));
    siftDown(0);
  }

  let total = 0;
  for (const g of heap) total += g;
  return total;
}
