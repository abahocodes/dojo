function countSubarraysMedianK(nums, k) {
  const n = nums.length;
  const p = nums.indexOf(k);
  // right[b + n + 1] counts right-side balances b (offset keeps indexes >= 0).
  const right = new Int32Array(2 * n + 3);
  const off = n + 1;
  let bal = 0;
  right[off]++;
  for (let i = p + 1; i < n; i++) {
    bal += nums[i] > k ? 1 : -1;
    right[bal + off]++;
  }
  let total = 0;
  bal = 0;
  for (let i = p; i >= 0; i--) {
    if (i < p) bal += nums[i] > k ? 1 : -1;
    total += right[-bal + off] + right[1 - bal + off];
  }
  return total;
}
