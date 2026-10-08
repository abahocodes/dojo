function findDuplicates(nums: number[]): number[] {
  const result: number[] = [];
  for (let i = 0; i < nums.length; i++) {
    const v = Math.abs(nums[i]);
    if (nums[v - 1] < 0) result.push(v);
    else nums[v - 1] = -nums[v - 1];
  }
  return result;
}
