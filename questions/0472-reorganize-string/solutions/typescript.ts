function reorganizeString(s: string): string {
  const counts: number[] = new Array(26).fill(0);
  for (let i = 0; i < s.length; i++) counts[s.charCodeAt(i) - 97]++;
  const n = s.length;
  if (Math.max(...counts) > Math.floor((n + 1) / 2)) return "";

  const result: string[] = [];
  let prev = -1;
  for (let pos = 0; pos < n; pos++) {
    const rest = n - pos - 1; // letters left after placing this one
    for (let c = 0; c < 26; c++) {
      if (counts[c] === 0 || c === prev) continue;
      counts[c]--;
      // The rest can follow c iff no letter needs more than half the
      // remaining slots, and c itself cannot take the very next slot.
      if (counts[c] <= Math.floor(rest / 2) && Math.max(...counts) <= Math.floor((rest + 1) / 2)) {
        result.push(String.fromCharCode(97 + c));
        prev = c;
        break;
      }
      counts[c]++;
    }
  }
  return result.join("");
}
