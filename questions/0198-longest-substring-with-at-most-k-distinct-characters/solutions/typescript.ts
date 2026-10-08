function lengthOfLongestSubstringKDistinct(s: string, k: number): number {
  const count: number[] = new Array(128).fill(0);
  let distinct = 0;
  let left = 0;
  let best = 0;
  for (let right = 0; right < s.length; right++) {
    if (count[s.charCodeAt(right)]++ === 0) distinct++;
    while (distinct > k) {
      if (--count[s.charCodeAt(left)] === 0) distinct--;
      left++;
    }
    best = Math.max(best, right - left + 1);
  }
  return best;
}
