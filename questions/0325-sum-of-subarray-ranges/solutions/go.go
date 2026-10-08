package main

func subArrayRanges(nums []int) int {
	return extremeSum(nums, 1) - extremeSum(nums, -1)
}

// extremeSum sums subarray maxima (sign = 1) or minima (sign = -1).
func extremeSum(nums []int, sign int) int {
	n := len(nums)
	stack := make([]int, 0, n)
	result := 0
	for i := 0; i <= n; i++ {
		for len(stack) > 0 && (i == n || sign*nums[stack[len(stack)-1]] <= sign*nums[i]) {
			j := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			left := -1
			if len(stack) > 0 {
				left = stack[len(stack)-1]
			}
			result += nums[j] * (j - left) * (i - j)
		}
		stack = append(stack, i)
	}
	return result
}
