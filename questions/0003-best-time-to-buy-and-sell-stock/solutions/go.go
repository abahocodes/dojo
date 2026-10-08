package main

func maxProfit(prices []int) int {
	lowest := prices[0]
	best := 0
	for _, p := range prices {
		best = max(best, p-lowest)
		lowest = min(lowest, p)
	}
	return best
}
