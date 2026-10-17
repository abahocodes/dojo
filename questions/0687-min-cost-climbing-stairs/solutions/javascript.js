function minCostClimbingStairs(cost) {
  // a, b = cheapest way to stand on step i-2, i-1 (steps 0 and 1 are free starts).
  let a = 0;
  let b = 0;
  for (let i = 2; i <= cost.length; i++) {
    const next = Math.min(b + cost[i - 1], a + cost[i - 2]);
    a = b;
    b = next;
  }
  return b;
}
