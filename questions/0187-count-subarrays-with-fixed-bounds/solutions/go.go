package main

func countSubarraysFixedBounds(nums []int, minK int, maxK int) int {
	total := 0
	bad, lastMin, lastMax := -1, -1, -1
	for i, v := range nums {
		if v < minK || v > maxK {
			bad = i
		}
		if v == minK {
			lastMin = i
		}
		if v == maxK {
			lastMax = i
		}
		if start := min(lastMin, lastMax); start > bad {
			total += start - bad
		}
	}
	return total
}
