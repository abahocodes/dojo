function rotate(nums: number[], k: number): number[] {
  const n = nums.length;
  k %= n;
  const reverse = (lo: number, hi: number): void => {
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
