package main

func isIdealPermutation(nums []int) bool {
	best := -1 // max of nums[0..j-2]
	for j := 2; j < len(nums); j++ {
		if nums[j-2] > best {
			best = nums[j-2]
		}
		if best > nums[j] {
			return false
		}
	}
	return true
}
