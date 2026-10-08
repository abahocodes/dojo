function findOriginalArray(changed) {
  if (changed.length % 2 !== 0) return [];
  let top = 0;
  for (const v of changed) top = Math.max(top, v);
  const count = new Int32Array(top + 1);
  for (const v of changed) count[v]++;
  if (count[0] % 2 !== 0) return [];
  const original = new Array(count[0] / 2).fill(0);
  for (let x = 1; x <= top; x++) {
    const c = count[x];
    if (c === 0) continue;
    if (2 * x > top || count[2 * x] < c) return [];
    count[2 * x] -= c;
    for (let k = 0; k < c; k++) original.push(x);
  }
  return original;
}
