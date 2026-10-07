function maxProfit(prices) {
  let lowest = prices[0];
  let best = 0;
  for (const p of prices) {
    best = Math.max(best, p - lowest);
    lowest = Math.min(lowest, p);
  }
  return best;
}
