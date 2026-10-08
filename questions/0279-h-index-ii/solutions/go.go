package main

func hIndexSorted(citations []int) int {
	n := len(citations)
	// Find the first index i where the n - i papers from i onward
	// all have at least n - i citations.
	lo, hi := 0, n
	for lo < hi {
		mid := (lo + hi) / 2
		if citations[mid] >= n-mid {
			hi = mid
		} else {
			lo = mid + 1
		}
	}
	return n - lo
}
