function climbStairs(n) {
  let prev = 1; // ways to reach step 0
  let curr = 1; // ways to reach step 1
  for (let i = 1; i < n; i++) {
    [prev, curr] = [curr, prev + curr];
  }
  return curr;
}
