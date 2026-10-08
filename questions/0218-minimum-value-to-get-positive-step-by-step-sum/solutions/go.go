package main

func minStartValue(nums []int) int {
	total, low := 0, 0
	for _, x := range nums {
		total += x
		if total < low {
			low = total
		}
	}
	return 1 - low
}
