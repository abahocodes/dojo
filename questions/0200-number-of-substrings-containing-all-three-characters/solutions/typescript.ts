function numberOfSubstrings(s: string): number {
  const last: number[] = [-1, -1, -1];
  let total = 0;
  for (let i = 0; i < s.length; i++) {
    last[s.charCodeAt(i) - 97] = i;
    total += Math.min(last[0], last[1], last[2]) + 1;
  }
  return total;
}
