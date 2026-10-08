function maxSubArray(nums: number[]): number {
  let current = nums[0];
  let best = nums[0];
  for (let i = 1; i < nums.length; i++) {
    const x = nums[i];
    current = Math.max(x, current + x);
    best = Math.max(best, current);
  }
  return best;
}
