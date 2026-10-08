function threeSum(nums: number[]): number[][] {
  const sorted = [...nums].sort((a, b) => a - b);
  const n = sorted.length;
  const result: number[][] = [];
  for (let i = 0; i < n - 2; i++) {
    const a = sorted[i];
    if (a > 0) break;
    if (i > 0 && a === sorted[i - 1]) continue;
    let lo = i + 1;
    let hi = n - 1;
    while (lo < hi) {
      const s = a + sorted[lo] + sorted[hi];
      if (s < 0) {
        lo++;
      } else if (s > 0) {
        hi--;
      } else {
        result.push([a, sorted[lo], sorted[hi]]);
        lo++;
        hi--;
        while (lo < hi && sorted[lo] === sorted[lo - 1]) lo++;
      }
    }
  }
  return result;
}
