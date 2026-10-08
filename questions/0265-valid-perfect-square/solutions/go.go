package main

func isPerfectSquare(num int) bool {
	lo, hi := 1, num
	if hi > 1<<26 {
		hi = 1 << 26
	}
	for lo <= hi {
		mid := (lo + hi) / 2
		square := mid * mid
		if square == num {
			return true
		}
		if square < num {
			lo = mid + 1
		} else {
			hi = mid - 1
		}
	}
	return false
}
