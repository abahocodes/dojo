package main

func findDuplicates(nums []int) []int {
	result := []int{}
	for i := range nums {
		v := nums[i]
		if v < 0 {
			v = -v
		}
		if nums[v-1] < 0 {
			result = append(result, v)
		} else {
			nums[v-1] = -nums[v-1]
		}
	}
	return result
}
