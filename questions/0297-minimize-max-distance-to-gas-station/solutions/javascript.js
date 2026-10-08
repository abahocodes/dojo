function minmaxGasDist(stations, k) {
  const gaps = [];
  let hi = 0;
  for (let i = 1; i < stations.length; i++) {
    gaps.push(stations[i] - stations[i - 1]);
    hi = Math.max(hi, stations[i] - stations[i - 1]);
  }
  const fits = (limit) => {
    let added = 0;
    for (const g of gaps) {
      added += Math.floor(g / limit);
      if (added > k) return false;
    }
    return true;
  };

  let lo = 0;
  for (let iter = 0; iter < 100; iter++) {
    const mid = (lo + hi) / 2;
    if (fits(mid)) hi = mid;
    else lo = mid;
  }
  return hi;
}
