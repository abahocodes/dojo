function maxScoreCards(cardPoints, k) {
  const n = cardPoints.length;
  let current = 0;
  for (let i = 0; i < k; i++) current += cardPoints[i];
  let best = current;
  for (let i = 1; i <= k; i++) {
    current += cardPoints[n - i] - cardPoints[k - i];
    best = Math.max(best, current);
  }
  return best;
}
