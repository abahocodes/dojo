package main

func maxValueAtIndex(n int, index int, maxSum int) int {
	side := func(v, length int) int {
		if length >= v-1 {
			return (v-1)*v/2 + (length - v + 1)
		}
		return length*v - length*(length+1)/2
	}
	lo, hi := 1, maxSum
	for lo < hi {
		mid := lo + (hi-lo+1)/2
		if mid+side(mid, index)+side(mid, n-1-index) <= maxSum {
			lo = mid
		} else {
			hi = mid - 1
		}
	}
	return lo
}
