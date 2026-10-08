function findErrorNums(nums: number[]): number[] {
  const n = nums.length;
  const seen: boolean[] = new Array(n + 1).fill(false);
  let dup = 0;
  let total = 0;
  for (const x of nums) {
    if (seen[x]) dup = x;
    seen[x] = true;
    total += x;
  }
  const missing = (n * (n + 1)) / 2 - (total - dup);
  return [dup, missing];
}
