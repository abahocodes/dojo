function countCombinations(coins, amount) {
  const ways = new Array(amount + 1).fill(0);
  ways[0] = 1;
  for (const c of coins) {
    for (let x = c; x <= amount; x++) {
      ways[x] += ways[x - c];
    }
  }
  return ways[amount];
}
