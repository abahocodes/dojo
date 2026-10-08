function fourSum(nums: number[], target: number): number[][] {
  const a = [...nums].sort((x, y) => x - y);
  const n = a.length;
  const res: number[][] = [];
  for (let i = 0; i < n - 3; i++) {
    if (i > 0 && a[i] === a[i - 1]) continue;
    for (let j = i + 1; j < n - 2; j++) {
      if (j > i + 1 && a[j] === a[j - 1]) continue;
      let lo = j + 1;
      let hi = n - 1;
      while (lo < hi) {
        // |sum| <= 4e9, exactly representable as a double.
        const s = a[i] + a[j] + a[lo] + a[hi];
        if (s < target) lo++;
        else if (s > target) hi--;
        else {
          res.push([a[i], a[j], a[lo], a[hi]]);
          lo++;
          while (lo < hi && a[lo] === a[lo - 1]) lo++;
          hi--;
          while (lo < hi && a[hi] === a[hi + 1]) hi--;
        }
      }
    }
  }
  return res;
}
