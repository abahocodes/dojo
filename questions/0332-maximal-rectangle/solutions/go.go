package main

func maximalRectangle(matrix []string) int {
	cols := len(matrix[0])
	heights := make([]int, cols+1) // heights[cols] stays 0
	stack := make([]int, 0, cols+1)
	best := 0
	for _, row := range matrix {
		for c := 0; c < cols; c++ {
			if row[c] == '1' {
				heights[c]++
			} else {
				heights[c] = 0
			}
		}
		stack = stack[:0]
		for i := 0; i <= cols; i++ {
			for len(stack) > 0 && heights[stack[len(stack)-1]] >= heights[i] {
				h := heights[stack[len(stack)-1]]
				stack = stack[:len(stack)-1]
				left := -1
				if len(stack) > 0 {
					left = stack[len(stack)-1]
				}
				if h*(i-left-1) > best {
					best = h * (i - left - 1)
				}
			}
			stack = append(stack, i)
		}
	}
	return best
}
