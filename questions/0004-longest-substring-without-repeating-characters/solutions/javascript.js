function lengthOfLongestSubstring(s) {
  const last = new Map();
  let left = 0;
  let best = 0;
  for (let i = 0; i < s.length; i++) {
    const ch = s[i];
    if (last.has(ch) && last.get(ch) >= left) left = last.get(ch) + 1;
    last.set(ch, i);
    best = Math.max(best, i - left + 1);
  }
  return best;
}
