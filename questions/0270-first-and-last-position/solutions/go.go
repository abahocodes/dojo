package main

import "sort"

func searchRange(nums []int, target int) []int {
	first := sort.Search(len(nums), func(i int) bool { return nums[i] >= target })
	if first == len(nums) || nums[first] != target {
		return []int{-1, -1}
	}
	last := sort.Search(len(nums), func(i int) bool { return nums[i] > target }) - 1
	return []int{first, last}
}
