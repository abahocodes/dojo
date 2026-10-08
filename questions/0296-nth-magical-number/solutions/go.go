package main

func nthMagicalNumber(n int, a int, b int) int {
	const mod = 1_000_000_007
	x, y := a, b
	for y != 0 {
		x, y = y, x%y
	}
	lcm := a / x * b
	small := a
	if b < small {
		small = b
	}
	lo, hi := small, n*small
	for lo < hi {
		mid := lo + (hi-lo)/2
		if mid/a+mid/b-mid/lcm >= n {
			hi = mid
		} else {
			lo = mid + 1
		}
	}
	return lo % mod
}
