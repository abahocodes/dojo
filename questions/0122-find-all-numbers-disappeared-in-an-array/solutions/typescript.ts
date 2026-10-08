function findDisappearedNumbers(nums: number[]): number[] {
  // Mark value v as seen by making nums[v - 1] negative.
  for (const x of nums) {
    const i = Math.abs(x) - 1;
    if (nums[i] > 0) nums[i] = -nums[i];
  }
  const out: number[] = [];
  for (let i = 0; i < nums.length; i++) {
    if (nums[i] > 0) out.push(i + 1);
  }
  return out;
}
