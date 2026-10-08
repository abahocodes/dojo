package main

func coinChange(coins []int, amount int) int {
	inf := amount + 1
	best := make([]int, amount+1)
	for i := 1; i <= amount; i++ {
		best[i] = inf
	}
	for total := 1; total <= amount; total++ {
		for _, coin := range coins {
			if coin <= total && best[total-coin]+1 < best[total] {
				best[total] = best[total-coin] + 1
			}
		}
	}
	if best[amount] == inf {
		return -1
	}
	return best[amount]
}
