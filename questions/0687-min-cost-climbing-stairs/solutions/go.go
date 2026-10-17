package main

func minCostClimbingStairs(cost []int) int {
	// a, b = cheapest way to stand on step i-2, i-1 (steps 0 and 1 are free starts).
	a, b := 0, 0
	for i := 2; i <= len(cost); i++ {
		next := b + cost[i-1]
		if alt := a + cost[i-2]; alt < next {
			next = alt
		}
		a, b = b, next
	}
	return b
}
