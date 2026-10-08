package main

import "strconv"

func summaryRanges(nums []int) []string {
	result := []string{}
	i := 0
	for i < len(nums) {
		j := i
		for j+1 < len(nums) && nums[j+1] == nums[j]+1 {
			j++
		}
		if i == j {
			result = append(result, strconv.Itoa(nums[i]))
		} else {
			result = append(result, strconv.Itoa(nums[i])+"->"+strconv.Itoa(nums[j]))
		}
		i = j + 1
	}
	return result
}
