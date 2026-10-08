function findOrder(numCourses: number, prerequisites: number[][]): number[] {
  const unlocks: number[][] = Array.from({ length: numCourses }, () => []);
  const indegree = new Array<number>(numCourses).fill(0);
  for (const [course, before] of prerequisites) {
    unlocks[before].push(course);
    indegree[course]++;
  }

  // A min-heap always hands out the smallest course that is ready now,
  // which gives the lexicographically smallest valid order.
  const heap: number[] = [];
  const push = (x: number): void => {
    heap.push(x);
    let i = heap.length - 1;
    while (i > 0) {
      const parent = (i - 1) >> 1;
      if (heap[parent] <= heap[i]) break;
      [heap[parent], heap[i]] = [heap[i], heap[parent]];
      i = parent;
    }
  };
  const pop = (): number => {
    const top = heap[0];
    const last = heap.pop()!;
    if (heap.length > 0) {
      heap[0] = last;
      let i = 0;
      for (;;) {
        const l = 2 * i + 1;
        const r = l + 1;
        let smallest = i;
        if (l < heap.length && heap[l] < heap[smallest]) smallest = l;
        if (r < heap.length && heap[r] < heap[smallest]) smallest = r;
        if (smallest === i) break;
        [heap[smallest], heap[i]] = [heap[i], heap[smallest]];
        i = smallest;
      }
    }
    return top;
  };

  for (let i = 0; i < numCourses; i++) if (indegree[i] === 0) push(i);
  const order: number[] = [];
  while (heap.length > 0) {
    const current = pop();
    order.push(current);
    for (const next of unlocks[current]) {
      indegree[next]--;
      if (indegree[next] === 0) push(next);
    }
  }
  return order.length === numCourses ? order : [];
}
