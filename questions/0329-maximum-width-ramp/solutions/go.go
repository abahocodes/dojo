package main

func maxWidthRamp(nums []int) int {
	stack := []int{}
	for i, x := range nums {
		if len(stack) == 0 || x < nums[stack[len(stack)-1]] {
			stack = append(stack, i)
		}
	}
	best := 0
	for j := len(nums) - 1; j >= 0; j-- {
		for len(stack) > 0 && nums[stack[len(stack)-1]] <= nums[j] {
			if j-stack[len(stack)-1] > best {
				best = j - stack[len(stack)-1]
			}
			stack = stack[:len(stack)-1]
		}
	}
	return best
}
