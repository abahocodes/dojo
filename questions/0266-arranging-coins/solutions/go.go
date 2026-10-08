package main

func arrangeCoins(n int) int {
	lo, hi := 0, n
	if hi > 94906266 {
		hi = 94906266
	}
	for lo < hi {
		mid := (lo + hi + 1) / 2
		if mid*(mid+1)/2 <= n {
			lo = mid
		} else {
			hi = mid - 1
		}
	}
	return lo
}
