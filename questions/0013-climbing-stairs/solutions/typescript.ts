function climbStairs(n: number): number {
  let prev = 1; // ways to reach step 0
  let curr = 1; // ways to reach step 1
  for (let i = 0; i < n - 1; i++) {
    [prev, curr] = [curr, prev + curr];
  }
  return curr;
}
