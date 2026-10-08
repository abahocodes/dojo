function maximumSubarraySumDistinct(nums, k) {
  const count = new Map();
  let dup = 0;
  let window = 0;
  let best = 0;
  for (let i = 0; i < nums.length; i++) {
    const value = nums[i];
    window += value;
    const c = (count.get(value) ?? 0) + 1;
    count.set(value, c);
    if (c === 2) dup++;
    if (i >= k) {
      const old = nums[i - k];
      window -= old;
      const d = count.get(old) - 1;
      count.set(old, d);
      if (d === 1) dup--;
    }
    if (i >= k - 1 && dup === 0) best = Math.max(best, window);
  }
  return best;
}
