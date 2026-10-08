package main

func findDisappearedNumbers(nums []int) []int {
	// Mark value v as seen by making nums[v-1] negative.
	for _, x := range nums {
		if x < 0 {
			x = -x
		}
		if nums[x-1] > 0 {
			nums[x-1] = -nums[x-1]
		}
	}
	out := []int{}
	for i, x := range nums {
		if x > 0 {
			out = append(out, i+1)
		}
	}
	return out
}
