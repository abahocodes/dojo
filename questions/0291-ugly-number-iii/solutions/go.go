package main

const uglyLimit = 2000000000

func nthUglyNumber(n int, a int, b int, c int) int {
	gcd := func(p, q int) int {
		for q != 0 {
			p, q = q, p%q
		}
		return p
	}
	lcm := func(p, q int) int {
		r := p / gcd(p, q)
		if r > (uglyLimit+1)/q {
			return uglyLimit + 1
		}
		return min(r*q, uglyLimit+1)
	}
	ab, ac, bc := lcm(a, b), lcm(a, c), lcm(b, c)
	abc := lcm(ab, c)
	lo, hi := 1, uglyLimit
	for lo < hi {
		mid := lo + (hi-lo)/2
		count := mid/a + mid/b + mid/c - mid/ab - mid/ac - mid/bc + mid/abc
		if count >= n {
			hi = mid
		} else {
			lo = mid + 1
		}
	}
	return lo
}
