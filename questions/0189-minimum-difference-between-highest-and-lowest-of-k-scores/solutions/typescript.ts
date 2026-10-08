function minimumDifference(nums: number[], k: number): number {
  const s = [...nums].sort((a, b) => a - b);
  let best = Infinity;
  for (let i = 0; i + k - 1 < s.length; i++) {
    best = Math.min(best, s[i + k - 1] - s[i]);
  }
  return best;
}
