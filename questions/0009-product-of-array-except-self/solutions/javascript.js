function productExceptSelf(nums) {
  const n = nums.length;
  const out = new Array(n).fill(1);
  let prefix = 1;
  for (let i = 0; i < n; i++) {
    out[i] = prefix;
    prefix *= nums[i];
  }
  let suffix = 1;
  for (let i = n - 1; i >= 0; i--) {
    out[i] *= suffix;
    suffix *= nums[i];
  }
  // Normalize -0 (from a negative times zero) to 0.
  return out.map((x) => x + 0);
}
