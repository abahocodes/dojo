function maxMinPower(stations, r, k) {
  const n = stations.length;
  // power[i] = sum of stations in [i - r, i + r], via a sliding window.
  const power = new Array(n).fill(0);
  let window = 0;
  for (let i = 0; i < Math.min(n, r + 1); i++) window += stations[i];
  for (let i = 0; i < n; i++) {
    power[i] = window;
    if (i + r + 1 < n) window += stations[i + r + 1];
    if (i - r >= 0) window -= stations[i - r];
  }

  const added = new Array(n + 1).fill(0);
  const feasible = (target) => {
    added.fill(0);
    let extra = 0;
    let used = 0;
    for (let i = 0; i < n; i++) {
      extra += added[i];
      const have = power[i] + extra;
      if (have < target) {
        const need = target - have;
        used += need;
        if (used > k) return false;
        extra += need;
        added[Math.min(n, i + 2 * r + 1)] -= need;
      }
    }
    return true;
  };

  let lo = power.reduce((a, b) => Math.min(a, b), Infinity);
  let hi = lo + k;
  while (lo < hi) {
    const mid = lo + Math.ceil((hi - lo) / 2);
    if (feasible(mid)) lo = mid;
    else hi = mid - 1;
  }
  return lo;
}
