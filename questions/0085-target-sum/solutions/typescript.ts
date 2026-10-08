function findTargetSumWays(nums: number[], target: number): number {
  let total = 0;
  for (const x of nums) total += x;
  if (Math.abs(target) > total || (total + target) % 2 !== 0) return 0;
  const goal = (total + target) / 2;
  const ways = new Array<number>(goal + 1).fill(0);
  ways[0] = 1;
  for (const x of nums) {
    for (let s = goal; s >= x; s--) {
      ways[s] += ways[s - x];
    }
  }
  return ways[goal];
}
