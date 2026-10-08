function rotate(nums, k) {
  const n = nums.length;
  k %= n;
  const reverse = (lo, hi) => {
    while (lo < hi) {
      const tmp = nums[lo];
      nums[lo++] = nums[hi];
      nums[hi--] = tmp;
    }
  };
  reverse(0, n - 1);
  reverse(0, k - 1);
  reverse(k, n - 1);
  return nums;
}
