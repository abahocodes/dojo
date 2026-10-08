package main

func splitArray(nums []int, k int) int {
	piecesNeeded := func(capacity int) int {
		pieces, current := 1, 0
		for _, x := range nums {
			if current+x > capacity {
				pieces++
				current = x
			} else {
				current += x
			}
		}
		return pieces
	}

	lo, hi := 0, 0
	for _, x := range nums {
		if x > lo {
			lo = x
		}
		hi += x
	}
	for lo < hi {
		mid := lo + (hi-lo)/2
		if piecesNeeded(mid) <= k {
			hi = mid
		} else {
			lo = mid + 1
		}
	}
	return lo
}
