function isIdealPermutation(nums) {
  let best = -1; // max of nums[0..j-2]
  for (let j = 2; j < nums.length; j++) {
    best = Math.max(best, nums[j - 2]);
    if (best > nums[j]) return false;
  }
  return true;
}
