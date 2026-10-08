function countSubarraysMaxK(nums: number[], k: number): number {
  let m = 0;
  for (const v of nums) if (v > m) m = v;
  let count = 0;
  let left = 0;
  let total = 0;
  for (const v of nums) {
    if (v === m) count++;
    while (count >= k) {
      if (nums[left] === m) count--;
      left++;
    }
    total += left;
  }
  return total;
}
