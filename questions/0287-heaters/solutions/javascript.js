function findRadius(houses, heaters) {
  const hs = [...heaters].sort((a, b) => a - b);
  let best = 0;
  for (const x of houses) {
    let lo = 0;
    let hi = hs.length;
    while (lo < hi) {
      const mid = (lo + hi) >> 1;
      if (hs[mid] < x) lo = mid + 1;
      else hi = mid;
    }
    let near = Infinity;
    if (lo < hs.length) near = hs[lo] - x;
    if (lo > 0) near = Math.min(near, x - hs[lo - 1]);
    best = Math.max(best, near);
  }
  return best;
}
