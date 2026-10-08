function nextPermutation(nums: number[]): number[] {
  const n = nums.length;
  let i = n - 2;
  while (i >= 0 && nums[i] >= nums[i + 1]) i--;
  if (i >= 0) {
    let j = n - 1;
    while (nums[j] <= nums[i]) j--;
    [nums[i], nums[j]] = [nums[j], nums[i]];
  }
  for (let lo = i + 1, hi = n - 1; lo < hi; lo++, hi--) {
    [nums[lo], nums[hi]] = [nums[hi], nums[lo]];
  }
  return nums;
}
