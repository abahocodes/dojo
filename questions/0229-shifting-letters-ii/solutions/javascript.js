function shiftingLetters(s, shifts) {
  const n = s.length;
  const diff = new Array(n + 1).fill(0);
  for (const [start, end, direction] of shifts) {
    const delta = direction === 1 ? 1 : -1;
    diff[start] += delta;
    diff[end + 1] -= delta;
  }
  const out = new Array(n);
  let net = 0;
  for (let i = 0; i < n; i++) {
    net += diff[i];
    const code = (((s.charCodeAt(i) - 97 + net) % 26) + 26) % 26;
    out[i] = String.fromCharCode(code + 97);
  }
  return out.join("");
}
