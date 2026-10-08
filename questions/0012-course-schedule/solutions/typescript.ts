function canFinish(numCourses: number, prerequisites: number[][]): boolean {
  const unlocks: number[][] = Array.from({ length: numCourses }, () => []);
  const indegree = new Array<number>(numCourses).fill(0);
  for (const [course, before] of prerequisites) {
    unlocks[before].push(course);
    indegree[course]++;
  }

  const ready: number[] = [];
  for (let i = 0; i < numCourses; i++) if (indegree[i] === 0) ready.push(i);
  let head = 0;
  while (head < ready.length) {
    const current = ready[head++];
    for (const nxt of unlocks[current]) {
      indegree[nxt]--;
      if (indegree[nxt] === 0) ready.push(nxt);
    }
  }
  return head === numCourses;
}
