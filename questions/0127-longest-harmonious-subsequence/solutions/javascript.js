function findLhs(nums) {
  const count = new Map();
  for (const x of nums) count.set(x, (count.get(x) || 0) + 1);
  let best = 0;
  for (const [x, c] of count) {
    if (count.has(x + 1)) best = Math.max(best, c + count.get(x + 1));
  }
  return best;
}
