function findSubstring(s, words) {
  const L = words[0].length;
  const m = words.length;
  if (m * L > s.length) return [];
  const need = new Map();
  for (const w of words) need.set(w, (need.get(w) || 0) + 1);
  const result = [];
  for (let r = 0; r < L; r++) {
    const have = new Map();
    let left = r;
    let count = 0;
    for (let right = r; right + L <= s.length; right += L) {
      const w = s.substr(right, L);
      if (!need.has(w)) {
        have.clear();
        count = 0;
        left = right + L;
        continue;
      }
      have.set(w, (have.get(w) || 0) + 1);
      count++;
      while (have.get(w) > need.get(w)) {
        const out = s.substr(left, L);
        have.set(out, have.get(out) - 1);
        count--;
        left += L;
      }
      if (count === m) {
        result.push(left);
        const out = s.substr(left, L);
        have.set(out, have.get(out) - 1);
        count--;
        left += L;
      }
    }
  }
  return result.sort((a, b) => a - b);
}
