package main

import "sort"

func minCost(nums []int, cost []int) int {
	n := len(nums)
	order := make([]int, n)
	total := 0
	for i := range order {
		order[i] = i
		total += cost[i]
	}
	sort.Slice(order, func(a, b int) bool { return nums[order[a]] < nums[order[b]] })
	acc := 0
	target := nums[order[0]]
	for _, i := range order {
		acc += cost[i]
		if 2*acc >= total {
			target = nums[i]
			break
		}
	}
	answer := 0
	for i := 0; i < n; i++ {
		d := nums[i] - target
		if d < 0 {
			d = -d
		}
		answer += cost[i] * d
	}
	return answer
}
