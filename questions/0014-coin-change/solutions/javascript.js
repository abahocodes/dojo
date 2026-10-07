function coinChange(coins, amount) {
  const INF = amount + 1;
  const best = new Array(amount + 1).fill(INF);
  best[0] = 0;
  for (let total = 1; total <= amount; total++) {
    for (const coin of coins) {
      if (coin <= total && best[total - coin] + 1 < best[total]) {
        best[total] = best[total - coin] + 1;
      }
    }
  }
  return best[amount] === INF ? -1 : best[amount];
}
