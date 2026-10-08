function sortedSquares(nums: number[]): number[] {
  const n = nums.length;
  const out: number[] = new Array(n);
  let lo = 0;
  let hi = n - 1;
  for (let w = n - 1; w >= 0; w--) {
    if (Math.abs(nums[lo]) > Math.abs(nums[hi])) {
      out[w] = nums[lo] * nums[lo];
      lo++;
    } else {
      out[w] = nums[hi] * nums[hi];
      hi--;
    }
  }
  return out;
}
