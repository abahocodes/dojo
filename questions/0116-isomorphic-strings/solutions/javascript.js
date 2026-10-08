function isIsomorphic(s, t) {
  const forward = new Map();
  const backward = new Map();
  for (let i = 0; i < s.length; i++) {
    const a = s[i];
    const b = t[i];
    if (forward.has(a) ? forward.get(a) !== b : backward.has(b)) return false;
    forward.set(a, b);
    backward.set(b, a);
  }
  return true;
}
