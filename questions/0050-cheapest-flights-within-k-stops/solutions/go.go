package main

import "math"

func findCheapestPrice(n int, flights [][]int, src int, dst int, k int) int {
	const inf = math.MaxInt
	cost := make([]int, n)
	for i := range cost {
		cost[i] = inf
	}
	cost[src] = 0
	// Round i relaxes every flight once, allowing paths of up to i flights.
	// At most k stops means at most k + 1 flights.
	for round := 0; round <= k; round++ {
		next := append([]int(nil), cost...) // read last round's costs so one round adds one flight
		for _, f := range flights {
			u, v, price := f[0], f[1], f[2]
			if cost[u] != inf && cost[u]+price < next[v] {
				next[v] = cost[u] + price
			}
		}
		cost = next
	}
	if cost[dst] == inf {
		return -1
	}
	return cost[dst]
}
