function minDays(bloomDay, m, k) {
  if (m * k > bloomDay.length) return -1;
  const bouquets = (day) => {
    let made = 0;
    let run = 0;
    for (const b of bloomDay) {
      if (b <= day) {
        run++;
        if (run === k) {
          made++;
          run = 0;
        }
      } else {
        run = 0;
      }
    }
    return made;
  };
  let lo = Infinity;
  let hi = -Infinity;
  for (const b of bloomDay) {
    lo = Math.min(lo, b);
    hi = Math.max(hi, b);
  }
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    if (bouquets(mid) >= m) hi = mid;
    else lo = mid + 1;
  }
  return lo;
}
