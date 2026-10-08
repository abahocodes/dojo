function shiftingLetters(s: string, shifts: number[][]): string {
  const n = s.length;
  const diff: number[] = new Array(n + 1).fill(0);
  for (const [start, end, direction] of shifts) {
    const delta = direction === 1 ? 1 : -1;
    diff[start] += delta;
    diff[end + 1] -= delta;
  }
  const out: string[] = new Array(n);
  let net = 0;
  for (let i = 0; i < n; i++) {
    net += diff[i];
    const code = (((s.charCodeAt(i) - 97 + net) % 26) + 26) % 26;
    out[i] = String.fromCharCode(code + 97);
  }
  return out.join("");
}
