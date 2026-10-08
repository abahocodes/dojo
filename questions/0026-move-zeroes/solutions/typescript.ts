function moveZeroes(nums: number[]): number[] {
  let w = 0;
  for (let read = 0; read < nums.length; read++) {
    if (nums[read] !== 0) {
      [nums[w], nums[read]] = [nums[read], nums[w]];
      w++;
    }
  }
  return nums;
}
