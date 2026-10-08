function smallestDivisor(nums: number[], threshold: number): number {
  const total = (d: number): number => {
    let s = 0;
    for (const x of nums) s += Math.ceil(x / d);
    return s;
  };
  let lo = 1;
  let hi = Math.max(...nums);
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    if (total(mid) <= threshold) hi = mid;
    else lo = mid + 1;
  }
  return lo;
}
