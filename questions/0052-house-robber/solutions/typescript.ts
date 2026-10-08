function rob(nums: number[]): number {
  let prev = 0;
  let curr = 0;
  for (const x of nums) {
    [prev, curr] = [curr, Math.max(curr, prev + x)];
  }
  return curr;
}
