function frequencySort(s: string): string {
  const counts: number[] = new Array(128).fill(0);
  for (let i = 0; i < s.length; i++) counts[s.charCodeAt(i)]++;
  const chars: number[] = [];
  for (let c = 0; c < 128; c++) if (counts[c] > 0) chars.push(c);
  chars.sort((a, b) => counts[b] - counts[a] || a - b);
  return chars.map((c) => String.fromCharCode(c).repeat(counts[c])).join("");
}
