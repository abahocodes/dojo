function maxOperations(nums: number[], k: number): number {
  const waiting = new Map<number, number>();
  let ops = 0;
  for (const x of nums) {
    const partner = k - x;
    const count = waiting.get(partner) ?? 0;
    if (count > 0) {
      waiting.set(partner, count - 1);
      ops++;
    } else {
      waiting.set(x, (waiting.get(x) ?? 0) + 1);
    }
  }
  return ops;
}
