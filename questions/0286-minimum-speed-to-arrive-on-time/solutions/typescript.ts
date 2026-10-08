function minSpeedOnTime(dist: number[], hour: number): number {
  const total = Math.round(hour * 100);
  const last = dist[dist.length - 1] * 100;
  const onTime = (speed: number): boolean => {
    let whole = 0;
    for (let i = 0; i < dist.length - 1; i++) whole += Math.ceil(dist[i] / speed);
    const rest = total - whole * 100;
    return rest >= 0 && (rest >= last || last <= rest * speed);
  };
  let lo = 1;
  let hi = 1e7;
  if (!onTime(hi)) return -1;
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    if (onTime(mid)) hi = mid;
    else lo = mid + 1;
  }
  return lo;
}
