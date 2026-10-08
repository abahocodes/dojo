function longestSubstringKRepeating(s: string, k: number): number {
  let best = 0;
  for (let limit = 1; limit <= 26; limit++) {
    const count = new Int32Array(26);
    let left = 0;
    let unique = 0;
    let atLeast = 0;
    for (let right = 0; right < s.length; right++) {
      const c = s.charCodeAt(right) - 97;
      if (count[c] === 0) unique++;
      count[c]++;
      if (count[c] === k) atLeast++;
      while (unique > limit) {
        const d = s.charCodeAt(left) - 97;
        if (count[d] === k) atLeast--;
        count[d]--;
        if (count[d] === 0) unique--;
        left++;
      }
      if (unique === atLeast) best = Math.max(best, right - left + 1);
    }
  }
  return best;
}
