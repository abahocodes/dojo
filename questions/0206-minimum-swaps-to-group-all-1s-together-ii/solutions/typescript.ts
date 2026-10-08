function minSwapsCircular(nums: number[]): number {
  const n = nums.length;
  let ones = 0;
  for (const v of nums) ones += v;
  if (ones === 0) return 0;
  let window = 0;
  for (let i = 0; i < ones; i++) window += nums[i];
  let best = window;
  for (let i = ones; i < ones + n - 1; i++) {
    window += nums[i % n] - nums[i - ones];
    best = Math.max(best, window);
  }
  return ones - best;
}
