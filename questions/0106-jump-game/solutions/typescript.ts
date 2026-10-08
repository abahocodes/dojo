function canJump(nums: number[]): boolean {
  let furthest = 0;
  const last = nums.length - 1;
  for (let i = 0; i < nums.length; i++) {
    if (i > furthest) return false;
    furthest = Math.max(furthest, i + nums[i]);
    if (furthest >= last) return true;
  }
  return true;
}
