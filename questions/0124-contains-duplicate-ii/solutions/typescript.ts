function containsNearbyDuplicate(nums: number[], k: number): boolean {
  const last = new Map<number, number>();
  for (let i = 0; i < nums.length; i++) {
    const x = nums[i];
    const j = last.get(x);
    if (j !== undefined && i - j <= k) return true;
    last.set(x, i);
  }
  return false;
}
