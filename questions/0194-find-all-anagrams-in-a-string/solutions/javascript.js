function findAnagrams(s, p) {
  const m = p.length;
  const result = [];
  if (m > s.length) return result;
  const need = new Array(26).fill(0);
  const have = new Array(26).fill(0);
  for (let j = 0; j < m; j++) need[p.charCodeAt(j) - 97]++;
  let matches = need.filter((x) => x === 0).length;
  const change = (i, delta) => {
    if (have[i] === need[i]) matches--;
    have[i] += delta;
    if (have[i] === need[i]) matches++;
  };
  for (let j = 0; j < s.length; j++) {
    change(s.charCodeAt(j) - 97, 1);
    if (j >= m) change(s.charCodeAt(j - m) - 97, -1);
    if (j >= m - 1 && matches === 26) result.push(j - m + 1);
  }
  return result;
}
