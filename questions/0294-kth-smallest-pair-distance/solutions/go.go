package main

import "sort"

func smallestDistancePair(nums []int, k int) int {
	sorted := append([]int(nil), nums...)
	sort.Ints(sorted)
	n := len(sorted)
	pairsWithin := func(limit int) int {
		count, left := 0, 0
		for right := 0; right < n; right++ {
			for sorted[right]-sorted[left] > limit {
				left++
			}
			count += right - left
		}
		return count
	}

	lo, hi := 0, sorted[n-1]-sorted[0]
	for lo < hi {
		mid := lo + (hi-lo)/2
		if pairsWithin(mid) >= k {
			hi = mid
		} else {
			lo = mid + 1
		}
	}
	return lo
}
