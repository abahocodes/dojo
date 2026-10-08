package main

func maximumSubarraySumDistinct(nums []int, k int) int {
	count := make([]int, 100001)
	dup, window, best := 0, 0, 0
	for i, value := range nums {
		window += value
		count[value]++
		if count[value] == 2 {
			dup++
		}
		if i >= k {
			old := nums[i-k]
			window -= old
			count[old]--
			if count[old] == 1 {
				dup--
			}
		}
		if i >= k-1 && dup == 0 && window > best {
			best = window
		}
	}
	return best
}
