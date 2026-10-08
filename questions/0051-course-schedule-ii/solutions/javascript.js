function findOrder(numCourses, prerequisites) {
  const unlocks = Array.from({ length: numCourses }, () => []);
  const indegree = new Array(numCourses).fill(0);
  for (const [course, before] of prerequisites) {
    unlocks[before].push(course);
    indegree[course]++;
  }

  // Minimal binary min-heap of course numbers.
  const heap = [];
  const push = (x) => {
    heap.push(x);
    let i = heap.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (heap[p] <= heap[i]) break;
      [heap[p], heap[i]] = [heap[i], heap[p]];
      i = p;
    }
  };
  const pop = () => {
    const top = heap[0];
    const last = heap.pop();
    if (heap.length > 0) {
      heap[0] = last;
      let i = 0;
      for (;;) {
        const l = 2 * i + 1;
        const r = l + 1;
        let m = i;
        if (l < heap.length && heap[l] < heap[m]) m = l;
        if (r < heap.length && heap[r] < heap[m]) m = r;
        if (m === i) break;
        [heap[m], heap[i]] = [heap[i], heap[m]];
        i = m;
      }
    }
    return top;
  };

  // The heap always hands out the smallest course that is ready now,
  // which gives the lexicographically smallest valid order.
  for (let i = 0; i < numCourses; i++) {
    if (indegree[i] === 0) push(i);
  }
  const order = [];
  while (heap.length > 0) {
    const current = pop();
    order.push(current);
    for (const next of unlocks[current]) {
      if (--indegree[next] === 0) push(next);
    }
  }
  return order.length === numCourses ? order : [];
}
