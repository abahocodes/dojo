package main

import "math/rand"

func minMovesToEqual(nums []int) int {
	a := append([]int(nil), nums...)
	k := len(a) / 2
	lo, hi := 0, len(a)-1
	for lo < hi {
		pivot := a[lo+rand.Intn(hi-lo+1)]
		// Three-way partition: [lo, lt) < pivot, [lt, gt] == pivot, (gt, hi] > pivot
		lt, i, gt := lo, lo, hi
		for i <= gt {
			if a[i] < pivot {
				a[lt], a[i] = a[i], a[lt]
				lt++
				i++
			} else if a[i] > pivot {
				a[i], a[gt] = a[gt], a[i]
				gt--
			} else {
				i++
			}
		}
		if k < lt {
			hi = lt - 1
		} else if k > gt {
			lo = gt + 1
		} else {
			break
		}
	}
	median := a[k]
	moves := 0
	for _, v := range a {
		if v > median {
			moves += v - median
		} else {
			moves += median - v
		}
	}
	return moves
}
