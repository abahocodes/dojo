package main

import "math"

func maxProfit(prices []int) int {
	// Best profit at the end of each day, by state:
	// hold = owning a share, sold = sold today, rest = no share and free to buy.
	hold, sold, rest := math.MinInt32, 0, 0
	for _, p := range prices {
		prevSold := sold
		sold = hold + p
		if rest-p > hold {
			hold = rest - p
		}
		if prevSold > rest {
			rest = prevSold
		}
	}
	if sold > rest {
		return sold
	}
	return rest
}
