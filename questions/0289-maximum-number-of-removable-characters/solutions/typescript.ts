function maximumRemovals(s: string, p: string, removable: number[]): number {
  const removedAt = new Int32Array(s.length).fill(removable.length);
  removable.forEach((i, step) => {
    removedAt[i] = step;
  });
  const survives = (k: number): boolean => {
    let j = 0;
    for (let i = 0; i < s.length && j < p.length; i++) {
      if (removedAt[i] >= k && s[i] === p[j]) j++;
    }
    return j === p.length;
  };
  let lo = 0;
  let hi = removable.length;
  while (lo < hi) {
    const mid = Math.floor((lo + hi + 1) / 2);
    if (survives(mid)) lo = mid;
    else hi = mid - 1;
  }
  return lo;
}
