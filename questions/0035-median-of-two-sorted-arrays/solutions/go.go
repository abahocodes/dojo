package main

import "math"

func findMedianSortedArrays(nums1 []int, nums2 []int) float64 {
	if len(nums1) > len(nums2) {
		nums1, nums2 = nums2, nums1
	}
	m, n := len(nums1), len(nums2)
	half := (m + n + 1) / 2
	inf := math.Inf(1)
	lo, hi := 0, m
	for lo <= hi {
		i := (lo + hi) / 2
		j := half - i
		left1, right1, left2, right2 := -inf, inf, -inf, inf
		if i > 0 {
			left1 = float64(nums1[i-1])
		}
		if i < m {
			right1 = float64(nums1[i])
		}
		if j > 0 {
			left2 = float64(nums2[j-1])
		}
		if j < n {
			right2 = float64(nums2[j])
		}
		if left1 > right2 {
			hi = i - 1
		} else if left2 > right1 {
			lo = i + 1
		} else {
			if (m+n)%2 == 1 {
				return math.Max(left1, left2)
			}
			return (math.Max(left1, left2) + math.Min(right1, right2)) / 2
		}
	}
	return 0
}
