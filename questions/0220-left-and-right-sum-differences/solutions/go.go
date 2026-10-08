package main

func leftRightDifference(nums []int) []int {
	total := 0
	for _, x := range nums {
		total += x
	}
	left := 0
	result := make([]int, len(nums))
	for i, x := range nums {
		right := total - left - x
		d := left - right
		if d < 0 {
			d = -d
		}
		result[i] = d
		left += x
	}
	return result
}
