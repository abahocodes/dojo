function findCheapestPrice(n: number, flights: number[][], src: number, dst: number, k: number): number {
  let cost = new Array<number>(n).fill(Infinity);
  cost[src] = 0;
  // Round i relaxes every flight once, allowing paths of up to i flights.
  // At most k stops means at most k + 1 flights.
  for (let round = 0; round <= k; round++) {
    const next = cost.slice(); // read last round's costs so one round adds one flight
    for (const [u, v, price] of flights) {
      if (cost[u] + price < next[v]) next[v] = cost[u] + price;
    }
    cost = next;
  }
  return cost[dst] === Infinity ? -1 : cost[dst];
}
