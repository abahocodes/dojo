package main

import "sort"

func numSubseq(nums []int, target int) int {
	const mod = 1_000_000_007
	a := append([]int(nil), nums...)
	sort.Ints(a)
	n := len(a)
	pow2 := make([]int, n)
	pow2[0] = 1
	for i := 1; i < n; i++ {
		pow2[i] = pow2[i-1] * 2 % mod
	}
	total := 0
	lo, hi := 0, n-1
	for lo <= hi {
		if a[lo]+a[hi] <= target {
			total = (total + pow2[hi-lo]) % mod
			lo++
		} else {
			hi--
		}
	}
	return total
}
