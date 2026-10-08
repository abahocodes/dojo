function minWindow(s, t) {
  const need = new Map();
  for (const ch of t) need.set(ch, (need.get(ch) || 0) + 1);
  let missing = t.length;
  let bestStart = 0;
  let bestLen = s.length + 1;
  let left = 0;
  for (let right = 0; right < s.length; right++) {
    const ch = s[right];
    const n = need.get(ch) || 0;
    if (n > 0) missing--;
    need.set(ch, n - 1);
    while (missing === 0) {
      if (right - left + 1 < bestLen) {
        bestStart = left;
        bestLen = right - left + 1;
      }
      const out = s[left];
      const m = need.get(out) + 1;
      need.set(out, m);
      if (m > 0) missing++;
      left++;
    }
  }
  return bestLen > s.length ? "" : s.slice(bestStart, bestStart + bestLen);
}
