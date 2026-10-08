package main

func kthSmallestProduct(nums1 []int, nums2 []int, k int) int {
	n2 := len(nums2)
	// Number of j with a * nums2[j] <= x.
	countFor := func(a, x int) int {
		if a == 0 {
			if x >= 0 {
				return n2
			}
			return 0
		}
		lo, hi := 0, n2
		if a > 0 {
			for lo < hi {
				mid := lo + (hi-lo)/2
				if a*nums2[mid] <= x {
					lo = mid + 1
				} else {
					hi = mid
				}
			}
			return lo
		}
		for lo < hi {
			mid := lo + (hi-lo)/2
			if a*nums2[mid] <= x {
				hi = mid
			} else {
				lo = mid + 1
			}
		}
		return n2 - lo
	}
	countAtMost := func(x int) int {
		total := 0
		for _, a := range nums1 {
			total += countFor(a, x)
		}
		return total
	}

	a0, a1 := nums1[0], nums1[len(nums1)-1]
	b0, b1 := nums2[0], nums2[n2-1]
	lo, hi := a0*b0, a0*b0
	for _, c := range []int{a0 * b1, a1 * b0, a1 * b1} {
		if c < lo {
			lo = c
		}
		if c > hi {
			hi = c
		}
	}
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
