package main

import "math"

func thirdMax(nums []int) int {
	// math.MinInt64 marks an empty slot: no 32-bit input can equal it.
	first, second, third := math.MinInt64, math.MinInt64, math.MinInt64
	for _, x := range nums {
		if x == first || x == second || x == third {
			continue
		}
		if x > first {
			first, second, third = x, first, second
		} else if x > second {
			second, third = x, second
		} else if x > third {
			third = x
		}
	}
	if third == math.MinInt64 {
		return first
	}
	return third
}
