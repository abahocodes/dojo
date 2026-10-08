package main

import "sort"

func frequencySortNumbers(nums []int) []int {
	var count [201]int
	for _, x := range nums {
		count[x+100]++
	}
	result := append([]int(nil), nums...)
	sort.Slice(result, func(i, j int) bool {
		a, b := result[i], result[j]
		if count[a+100] != count[b+100] {
			return count[a+100] < count[b+100]
		}
		return a > b
	})
	return result
}
