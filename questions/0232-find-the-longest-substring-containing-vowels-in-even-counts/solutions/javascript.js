function findTheLongestSubstring(s) {
  const bit = { a: 1, e: 2, i: 4, o: 8, u: 16 };
  const first = new Array(32).fill(-2);
  first[0] = -1;
  let mask = 0;
  let best = 0;
  for (let i = 0; i < s.length; i++) {
    mask ^= bit[s[i]] || 0;
    if (first[mask] === -2) first[mask] = i;
    else best = Math.max(best, i - first[mask]);
  }
  return best;
}
