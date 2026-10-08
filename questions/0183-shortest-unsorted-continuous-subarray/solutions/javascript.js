function findUnsortedSubarray(nums) {
  const n = nums.length;
  let end = -1;
  let runningMax = nums[0];
  for (let i = 1; i < n; i++) {
    if (nums[i] < runningMax) end = i;
    else runningMax = nums[i];
  }
  if (end === -1) return 0;
  let start = n;
  let runningMin = nums[n - 1];
  for (let i = n - 2; i >= 0; i--) {
    if (nums[i] > runningMin) start = i;
    else runningMin = nums[i];
  }
  return end - start + 1;
}
