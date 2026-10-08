function minimumTime(time, totalTrips) {
  const enough = (t) => {
    let done = 0;
    for (const x of time) {
      done += Math.floor(t / x);
      if (done >= totalTrips) return true;
    }
    return false;
  };
  let lo = 1;
  let hi = Math.min(...time) * totalTrips;
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    if (enough(mid)) hi = mid;
    else lo = mid + 1;
  }
  return lo;
}
