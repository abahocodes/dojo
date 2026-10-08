package main

func maxIceCream(costs []int, coins int) int {
	maxCost := 0
	for _, c := range costs {
		maxCost = max(maxCost, c)
	}
	count := make([]int, maxCost+1)
	for _, c := range costs {
		count[c]++
	}
	bought := 0
	for price := 1; price <= maxCost; price++ {
		if count[price] == 0 {
			continue
		}
		take := min(count[price], coins/price)
		bought += take
		coins -= take * price
		if take < count[price] {
			break
		}
	}
	return bought
}
