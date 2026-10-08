function findLhs(nums: number[]): number {
  const count = new Map<number, number>();
  for (const x of nums) count.set(x, (count.get(x) ?? 0) + 1);
  let best = 0;
  for (const [x, c] of count) {
    const next = count.get(x + 1);
    if (next !== undefined) best = Math.max(best, c + next);
  }
  return best;
}
