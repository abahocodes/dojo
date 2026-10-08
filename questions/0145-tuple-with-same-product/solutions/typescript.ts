function tupleSameProduct(nums: number[]): number {
  const seen = new Map<number, number>();
  let total = 0;
  for (let i = 0; i < nums.length; i++) {
    for (let j = i + 1; j < nums.length; j++) {
      const p = nums[i] * nums[j];
      const count = seen.get(p) ?? 0;
      total += 8 * count;
      seen.set(p, count + 1);
    }
  }
  return total;
}
