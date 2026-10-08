function firstUniqChar(s: string): number {
  const counts: number[] = new Array(26).fill(0);
  for (let i = 0; i < s.length; i++) counts[s.charCodeAt(i) - 97]++;
  for (let i = 0; i < s.length; i++) {
    if (counts[s.charCodeAt(i) - 97] === 1) return i;
  }
  return -1;
}
