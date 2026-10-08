function heightChecker(heights: number[]): number {
  const count: number[] = new Array(101).fill(0);
  for (const h of heights) count[h]++;
  let mismatches = 0;
  let expected = 1;
  for (const h of heights) {
    while (count[expected] === 0) expected++;
    if (h !== expected) mismatches++;
    count[expected]--;
  }
  return mismatches;
}
