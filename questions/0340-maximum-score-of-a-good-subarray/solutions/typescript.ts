function maximumScore(nums: number[], k: number): number {
  const n = nums.length;
  let i = k;
  let j = k;
  let low = nums[k];
  let best = low;
  while (i > 0 || j < n - 1) {
    if (i === 0 || (j < n - 1 && nums[j + 1] > nums[i - 1])) {
      j++;
      low = Math.min(low, nums[j]);
    } else {
      i--;
      low = Math.min(low, nums[i]);
    }
    best = Math.max(best, low * (j - i + 1));
  }
  return best;
}
