function longestAwesome(s) {
  const n = s.length;
  const first = new Int32Array(1024).fill(n + 1);
  first[0] = 0;
  let mask = 0;
  let best = 0;
  for (let i = 1; i <= n; i++) {
    mask ^= 1 << (s.charCodeAt(i - 1) - 48);
    best = Math.max(best, i - first[mask]);
    for (let d = 0; d < 10; d++) {
      best = Math.max(best, i - first[mask ^ (1 << d)]);
    }
    if (first[mask] > i) first[mask] = i;
  }
  return best;
}
