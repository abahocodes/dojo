function mergeKSortedLists(lists) {
  if (!lists || lists.length === 0) return null;

  const heap = [];

  function push(node) {
    heap.push(node);
    let i = heap.length - 1;
    while (i > 0) {
      const parent = (i - 1) >> 1;
      if (heap[parent].val <= heap[i].val) break;
      [heap[parent], heap[i]] = [heap[i], heap[parent]];
      i = parent;
    }
  }

  function pop() {
    const top = heap[0];
    const last = heap.pop();
    if (heap.length > 0) {
      heap[0] = last;
      let i = 0;
      const n = heap.length;
      while (true) {
        let smallest = i;
        const l = 2 * i + 1;
        const r = 2 * i + 2;
        if (l < n && heap[l].val < heap[smallest].val) smallest = l;
        if (r < n && heap[r].val < heap[smallest].val) smallest = r;
        if (smallest === i) break;
        [heap[smallest], heap[i]] = [heap[i], heap[smallest]];
        i = smallest;
      }
    }
    return top;
  }

  for (const node of lists) {
    if (node) push(node);
  }

  const dummy = { val: 0, next: null };
  let curr = dummy;
  while (heap.length > 0) {
    const node = pop();
    curr.next = node;
    curr = curr.next;
    if (node.next) push(node.next);
  }
  return dummy.next;
}
