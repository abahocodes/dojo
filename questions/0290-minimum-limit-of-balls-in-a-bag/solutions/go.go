package main

func minimumSize(nums []int, maxOperations int) int {
	lo, hi := 1, 0
	for _, b := range nums {
		if b > hi {
			hi = b
		}
	}
	for lo < hi {
		mid := lo + (hi-lo)/2
		ops := 0
		for _, b := range nums {
			ops += (b - 1) / mid
			if ops > maxOperations {
				break
			}
		}
		if ops <= maxOperations {
			hi = mid
		} else {
			lo = mid + 1
		}
	}
	return lo
}
