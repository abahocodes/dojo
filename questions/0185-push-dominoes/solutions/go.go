package main

func pushDominoes(dominoes string) string {
	n := len(dominoes)
	res := []byte(dominoes)
	// Virtual 'L' before the row and 'R' after it never push anything inward.
	prevIdx, prev := -1, byte('L')
	for j := 0; j <= n; j++ {
		cur := byte('R')
		if j < n {
			cur = dominoes[j]
		}
		if cur == '.' {
			continue
		}
		if prev == cur {
			for k := prevIdx + 1; k < j; k++ {
				res[k] = cur
			}
		} else if prev == 'R' && cur == 'L' {
			for lo, hi := prevIdx+1, j-1; lo < hi; lo, hi = lo+1, hi-1 {
				res[lo] = 'R'
				res[hi] = 'L'
			}
		}
		prevIdx, prev = j, cur
	}
	return string(res)
}
