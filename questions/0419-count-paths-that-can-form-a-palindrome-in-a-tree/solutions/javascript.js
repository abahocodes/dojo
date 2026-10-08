function countPalindromePaths(parent, s) {
  const n = parent.length;
  const children = Array.from({ length: n }, () => []);
  for (let v = 1; v < n; v++) children[parent[v]].push(v);
  const mask = new Int32Array(n);
  const order = [0];
  for (let i = 0; i < order.length; i++) {
    const v = order[i];
    for (const c of children[v]) {
      mask[c] = mask[v] ^ (1 << (s.charCodeAt(c) - 97));
      order.push(c);
    }
  }
  const seen = new Map();
  let total = 0;
  for (let v = 0; v < n; v++) {
    const m = mask[v];
    total += seen.get(m) || 0;
    for (let b = 0; b < 26; b++) total += seen.get(m ^ (1 << b)) || 0;
    seen.set(m, (seen.get(m) || 0) + 1);
  }
  return total;
}
