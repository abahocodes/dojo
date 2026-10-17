function maxProfit(prices: number[]): number {
  // Best profit at the end of each day, by state:
  // hold = owning a share, sold = sold today, rest = no share and free to buy.
  let hold = -Infinity;
  let sold = 0;
  let rest = 0;
  for (const p of prices) {
    const prevSold = sold;
    sold = hold + p;
    hold = Math.max(hold, rest - p);
    rest = Math.max(rest, prevSold);
  }
  return Math.max(sold, rest);
}
