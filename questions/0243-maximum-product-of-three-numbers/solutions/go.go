package main

import "math"

func maximumProductThree(nums []int) int {
	max1, max2, max3 := math.MinInt, math.MinInt, math.MinInt
	min1, min2 := math.MaxInt, math.MaxInt
	for _, x := range nums {
		if x >= max1 {
			max1, max2, max3 = x, max1, max2
		} else if x >= max2 {
			max2, max3 = x, max2
		} else if x > max3 {
			max3 = x
		}
		if x <= min1 {
			min1, min2 = x, min1
		} else if x < min2 {
			min2 = x
		}
	}
	return max(max1*max2*max3, max1*min1*min2)
}
