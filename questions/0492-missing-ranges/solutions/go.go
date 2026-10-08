package main

func findMissingRanges(nums []int, lower int, upper int) [][]int {
	ranges := [][]int{}
	prev := lower - 1 // last value known to be present (a virtual one before lower)
	for i := 0; i <= len(nums); i++ {
		x := upper + 1
		if i < len(nums) {
			x = nums[i]
		}
		if x-prev >= 2 {
			ranges = append(ranges, []int{prev + 1, x - 1})
		}
		prev = x
	}
	return ranges
}
