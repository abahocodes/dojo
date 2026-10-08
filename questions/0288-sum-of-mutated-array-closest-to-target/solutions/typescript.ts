function findBestValue(arr: number[], target: number): number {
  const capped = (v: number): number => {
    let s = 0;
    for (const a of arr) s += a < v ? a : v;
    return s;
  };
  let lo = 0;
  let hi = 0;
  for (const a of arr) if (a > hi) hi = a;
  if (capped(hi) < target) return hi;
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    if (capped(mid) >= target) hi = mid;
    else lo = mid + 1;
  }
  if (lo > 0 && target - capped(lo - 1) <= capped(lo) - target) return lo - 1;
  return lo;
}
