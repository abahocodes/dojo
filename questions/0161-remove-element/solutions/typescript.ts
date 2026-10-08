function removeElement(nums: number[], val: number): number[] {
  let write = 0;
  for (const x of nums) {
    if (x !== val) nums[write++] = x;
  }
  return nums.slice(0, write);
}
