function minOperationsContinuous(nums: number[]): number {
  const n = nums.length;
  const u = Array.from(new Set(nums)).sort((a, b) => a - b);
  let best = 0;
  let j = 0;
  for (let i = 0; i < u.length; i++) {
    while (j < u.length && u[j] <= u[i] + n - 1) j++;
    best = Math.max(best, j - i);
  }
  return n - best;
}
