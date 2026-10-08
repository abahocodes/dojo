function arrayPairSum(nums: number[]): number {
  const a = [...nums].sort((x, y) => x - y);
  let total = 0;
  for (let i = 0; i < a.length; i += 2) total += a[i];
  return total;
}
