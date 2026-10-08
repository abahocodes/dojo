def find_cheapest_price(n: int, flights: list[list[int]], src: int, dst: int, k: int) -> int:
    inf = float("inf")
    cost = [inf] * n
    cost[src] = 0
    # Round i relaxes every flight once, allowing paths of up to i flights.
    # At most k stops means at most k + 1 flights.
    for _ in range(k + 1):
        nxt = cost[:]  # read last round's costs so one round adds one flight
        for u, v, price in flights:
            if cost[u] + price < nxt[v]:
                nxt[v] = cost[u] + price
        cost = nxt
    return -1 if cost[dst] == inf else cost[dst]
