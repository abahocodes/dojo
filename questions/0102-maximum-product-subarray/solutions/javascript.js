function maxProduct(nums) {
  let best = nums[0];
  let hi = nums[0];
  let lo = nums[0];
  for (let i = 1; i < nums.length; i++) {
    const x = nums[i];
    if (x < 0) [hi, lo] = [lo, hi];
    hi = Math.max(x, hi * x);
    lo = Math.min(x, lo * x);
    best = Math.max(best, hi);
  }
  return best;
}
