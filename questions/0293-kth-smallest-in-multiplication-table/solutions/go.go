package main

func findKthNumber(m int, n int, k int) int {
	if m > n {
		m, n = n, m
	}
	countAtMost := func(x int) int {
		total := 0
		for i := 1; i <= m; i++ {
			inRow := x / i
			if inRow == 0 {
				break
			}
			if inRow > n {
				inRow = n
			}
			total += inRow
		}
		return total
	}

	lo, hi := 1, m*n
	for lo < hi {
		mid := lo + (hi-lo)/2
		if countAtMost(mid) >= k {
			hi = mid
		} else {
			lo = mid + 1
		}
	}
	return lo
}
