function minimizedMaximum(n, quantities) {
  const storesNeeded = (cap) => {
    let s = 0;
    for (const q of quantities) s += Math.ceil(q / cap);
    return s;
  };
  let lo = 1;
  let hi = Math.max(...quantities);
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    if (storesNeeded(mid) <= n) hi = mid;
    else lo = mid + 1;
  }
  return lo;
}
