function kClosest(points, k) {
  // Binary max-heap of [squaredDistance, index], holding the k closest so far.
  const heap = [];
  const push = (item) => {
    heap.push(item);
    let i = heap.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (heap[p][0] >= heap[i][0]) break;
      [heap[p], heap[i]] = [heap[i], heap[p]];
      i = p;
    }
  };
  const replaceTop = (item) => {
    heap[0] = item;
    let i = 0;
    for (;;) {
      const l = 2 * i + 1;
      const r = l + 1;
      let m = i;
      if (l < heap.length && heap[l][0] > heap[m][0]) m = l;
      if (r < heap.length && heap[r][0] > heap[m][0]) m = r;
      if (m === i) break;
      [heap[m], heap[i]] = [heap[i], heap[m]];
      i = m;
    }
  };

  for (let i = 0; i < points.length; i++) {
    const [x, y] = points[i];
    const d = x * x + y * y;
    if (heap.length < k) push([d, i]);
    else if (d < heap[0][0]) replaceTop([d, i]);
  }
  return heap.map(([, i]) => points[i]);
}
