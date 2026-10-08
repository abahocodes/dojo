function minWindow(s: string, t: string): string {
  const need = new Map<string, number>();
  for (const ch of t) need.set(ch, (need.get(ch) ?? 0) + 1);
  let missing = t.length;
  let bestStart = 0;
  let bestLen = s.length + 1;
  let left = 0;
  for (let right = 0; right < s.length; right++) {
    const ch = s[right];
    const have = need.get(ch) ?? 0;
    if (have > 0) missing--;
    need.set(ch, have - 1);
    while (missing === 0) {
      if (right - left + 1 < bestLen) {
        bestStart = left;
        bestLen = right - left + 1;
      }
      const out = s[left];
      const next = (need.get(out) ?? 0) + 1;
      need.set(out, next);
      if (next > 0) missing++;
      left++;
    }
  }
  if (bestLen > s.length) return "";
  return s.slice(bestStart, bestStart + bestLen);
}
