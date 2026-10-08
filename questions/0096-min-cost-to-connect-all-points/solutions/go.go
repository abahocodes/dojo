package main

import "math"

func minCostConnectPoints(points [][]int) int {
	n := len(points)
	best := make([]int, n) // cheapest known edge from the tree to each point
	for i := range best {
		best[i] = math.MaxInt
	}
	inTree := make([]bool, n)
	best[0] = 0
	total := 0
	for step := 0; step < n; step++ {
		u, cost := -1, math.MaxInt
		for v := 0; v < n; v++ {
			if !inTree[v] && best[v] < cost {
				u, cost = v, best[v]
			}
		}
		inTree[u] = true
		total += cost
		ux, uy := points[u][0], points[u][1]
		for v := 0; v < n; v++ {
			if !inTree[v] {
				d := abs(points[v][0]-ux) + abs(points[v][1]-uy)
				if d < best[v] {
					best[v] = d
				}
			}
		}
	}
	return total
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}
