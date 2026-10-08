function removeDuplicates(nums: number[]): number[] {
  let write = 1;
  for (let read = 1; read < nums.length; read++) {
    if (nums[read] !== nums[write - 1]) {
      nums[write++] = nums[read];
    }
  }
  return nums.slice(0, write);
}
