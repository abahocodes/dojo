function maximumCandies(candies, k) {
  let lo = 0;
  let hi = 0;
  for (const c of candies) if (c > hi) hi = c;
  while (lo < hi) {
    const mid = Math.floor((lo + hi + 1) / 2);
    let shares = 0;
    for (const c of candies) {
      shares += Math.floor(c / mid);
      if (shares >= k) break;
    }
    if (shares >= k) lo = mid;
    else hi = mid - 1;
  }
  return lo;
}
