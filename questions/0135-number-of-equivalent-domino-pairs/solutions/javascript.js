function numEquivDominoPairs(dominoes) {
  const seen = new Array(100).fill(0);
  let pairs = 0;
  for (const [a, b] of dominoes) {
    const key = 10 * Math.min(a, b) + Math.max(a, b);
    pairs += seen[key];
    seen[key]++;
  }
  return pairs;
}
