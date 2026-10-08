function minOperationsReduceX(nums, x) {
  let total = 0;
  for (const v of nums) total += v;
  const target = total - x;
  if (target < 0) return -1;
  let best = -1;
  let window = 0;
  let left = 0;
  for (let right = 0; right < nums.length; right++) {
    window += nums[right];
    while (window > target) window -= nums[left++];
    if (window === target) best = Math.max(best, right - left + 1);
  }
  return best === -1 ? -1 : nums.length - best;
}
