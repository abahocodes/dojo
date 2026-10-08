function shipWithinDays(weights, days) {
  const daysNeeded = (cap) => {
    let used = 1;
    let load = 0;
    for (const w of weights) {
      if (load + w > cap) {
        used++;
        load = 0;
      }
      load += w;
    }
    return used;
  };
  let lo = 0;
  let hi = 0;
  for (const w of weights) {
    lo = Math.max(lo, w);
    hi += w;
  }
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    if (daysNeeded(mid) <= days) hi = mid;
    else lo = mid + 1;
  }
  return lo;
}
