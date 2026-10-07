function canFinish(numCourses, prerequisites) {
  const unlocks = Array.from({ length: numCourses }, () => []);
  const indegree = new Array(numCourses).fill(0);
  for (const [course, before] of prerequisites) {
    unlocks[before].push(course);
    indegree[course]++;
  }

  // Array used as a FIFO queue with a moving head index.
  const ready = [];
  for (let i = 0; i < numCourses; i++) {
    if (indegree[i] === 0) ready.push(i);
  }
  let head = 0;
  while (head < ready.length) {
    const current = ready[head++];
    for (const next of unlocks[current]) {
      if (--indegree[next] === 0) ready.push(next);
    }
  }
  return head === numCourses;
}
