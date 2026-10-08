package main

func nextGreaterElement(nums1 []int, nums2 []int) []int {
	next := make(map[int]int, len(nums2))
	stack := []int{} // decreasing values awaiting a greater one
	for _, x := range nums2 {
		for len(stack) > 0 && stack[len(stack)-1] < x {
			next[stack[len(stack)-1]] = x
			stack = stack[:len(stack)-1]
		}
		stack = append(stack, x)
	}
	result := make([]int, len(nums1))
	for i, x := range nums1 {
		if v, ok := next[x]; ok {
			result[i] = v
		} else {
			result[i] = -1
		}
	}
	return result
}
