function checkInclusion(s1: string, s2: string): boolean {
  const m = s1.length;
  if (m > s2.length) return false;
  const need: number[] = new Array(26).fill(0);
  const have: number[] = new Array(26).fill(0);
  for (let j = 0; j < m; j++) need[s1.charCodeAt(j) - 97]++;
  let matches = need.filter((x) => x === 0).length;
  const change = (i: number, delta: number): void => {
    if (have[i] === need[i]) matches--;
    have[i] += delta;
    if (have[i] === need[i]) matches++;
  };
  for (let j = 0; j < s2.length; j++) {
    change(s2.charCodeAt(j) - 97, 1);
    if (j >= m) change(s2.charCodeAt(j - m) - 97, -1);
    if (j >= m - 1 && matches === 26) return true;
  }
  return false;
}
