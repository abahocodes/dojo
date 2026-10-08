function lengthOfLongestSubstring(s: string): number {
  const last = new Map<string, number>();
  let left = 0;
  let best = 0;
  for (let i = 0; i < s.length; i++) {
    const prev = last.get(s[i]);
    if (prev !== undefined && prev >= left) left = prev + 1;
    last.set(s[i], i);
    best = Math.max(best, i - left + 1);
  }
  return best;
}
