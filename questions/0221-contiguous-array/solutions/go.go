package main

func findMaxLength(nums []int) int {
	n := len(nums)
	// balance ranges over [-n, n]; store first index at balance + n
	first := make([]int, 2*n+1)
	for i := range first {
		first[i] = -2
	}
	first[n] = -1
	balance, best := 0, 0
	for i, x := range nums {
		if x == 1 {
			balance++
		} else {
			balance--
		}
		slot := balance + n
		if first[slot] != -2 {
			if i-first[slot] > best {
				best = i - first[slot]
			}
		} else {
			first[slot] = i
		}
	}
	return best
}
