function orderlyQueue(s: string, k: number): string {
  if (k === 1) {
    const doubled = s + s;
    let best = s;
    for (let i = 1; i < s.length; i++) {
      const cand = doubled.slice(i, i + s.length);
      if (cand < best) best = cand;
    }
    return best;
  }
  return s.split("").sort().join("");
}
