package main

func maxChunksToSorted(arr []int) int {
	stack := make([]int, 0, len(arr)) // maximum of each chunk, non-decreasing
	for _, x := range arr {
		if len(stack) == 0 || x >= stack[len(stack)-1] {
			stack = append(stack, x)
		} else {
			biggest := stack[len(stack)-1]
			for len(stack) > 0 && stack[len(stack)-1] > x {
				stack = stack[:len(stack)-1]
			}
			stack = append(stack, biggest)
		}
	}
	return len(stack)
}
