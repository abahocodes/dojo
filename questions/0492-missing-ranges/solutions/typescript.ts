function findMissingRanges(nums: number[], lower: number, upper: number): number[][] {
  const ranges: number[][] = [];
  let prev = lower - 1; // last value known to be present (a virtual one before lower)
  for (let i = 0; i <= nums.length; i++) {
    const x = i < nums.length ? nums[i] : upper + 1;
    if (x - prev >= 2) ranges.push([prev + 1, x - 1]);
    prev = x;
  }
  return ranges;
}
