function threeSumClosest(nums, target) {
  const a = [...nums].sort((x, y) => x - y);
  const n = a.length;
  let best = a[0] + a[1] + a[2];
  for (let i = 0; i < n - 2; i++) {
    let lo = i + 1;
    let hi = n - 1;
    while (lo < hi) {
      const s = a[i] + a[lo] + a[hi];
      if (Math.abs(s - target) < Math.abs(best - target)) best = s;
      if (s < target) lo++;
      else if (s > target) hi--;
      else return s;
    }
  }
  return best;
}
