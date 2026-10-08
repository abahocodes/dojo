function maxRunTime(n: number, batteries: number[]): number {
  let total = 0;
  for (const b of batteries) total += b;
  const canRun = (minutes: number): boolean => {
    let usable = 0;
    for (const b of batteries) usable += Math.min(b, minutes);
    return usable >= n * minutes;
  };

  let lo = 0;
  let hi = Math.floor(total / n);
  while (lo < hi) {
    const mid = Math.floor((lo + hi + 1) / 2);
    if (canRun(mid)) lo = mid;
    else hi = mid - 1;
  }
  return lo;
}
