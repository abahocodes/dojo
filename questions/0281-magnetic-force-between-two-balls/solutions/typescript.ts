function maxMinDistance(position: number[], m: number): number {
  const pos = [...position].sort((a, b) => a - b);
  const fits = (gap: number): boolean => {
    // Greedily drop a ball in the leftmost basket at least `gap` past the last one.
    let placed = 1;
    let last = pos[0];
    for (let i = 1; i < pos.length; i++) {
      if (pos[i] - last >= gap) {
        placed++;
        last = pos[i];
        if (placed === m) return true;
      }
    }
    return false;
  };
  let lo = 1;
  let hi = Math.floor((pos[pos.length - 1] - pos[0]) / (m - 1));
  while (lo < hi) {
    const mid = Math.floor((lo + hi + 1) / 2);
    if (fits(mid)) lo = mid;
    else hi = mid - 1;
  }
  return lo;
}
