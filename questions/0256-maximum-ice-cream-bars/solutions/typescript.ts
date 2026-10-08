function maxIceCream(costs: number[], coins: number): number {
  let maxCost = 0;
  for (const c of costs) maxCost = Math.max(maxCost, c);
  const count = new Int32Array(maxCost + 1);
  for (const c of costs) count[c]++;
  let bought = 0;
  for (let price = 1; price <= maxCost; price++) {
    if (count[price] === 0) continue;
    const take = Math.min(count[price], Math.floor(coins / price));
    bought += take;
    coins -= take * price;
    if (take < count[price]) break;
  }
  return bought;
}
