function longestPalindrome(s: string): string {
  let bestLo = 0;
  let bestLen = 1;
  for (let center = 0; center < s.length; center++) {
    for (const [startLo, startHi] of [[center, center], [center, center + 1]]) {
      let lo = startLo;
      let hi = startHi;
      while (lo >= 0 && hi < s.length && s[lo] === s[hi]) {
        lo--;
        hi++;
      }
      const length = hi - lo - 1;
      if (length > bestLen) {
        bestLo = lo + 1;
        bestLen = length;
      }
    }
  }
  return s.slice(bestLo, bestLo + bestLen);
}
