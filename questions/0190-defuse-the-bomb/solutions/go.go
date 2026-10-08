package main

func decrypt(code []int, k int) []int {
	n := len(code)
	result := make([]int, n)
	if k == 0 {
		return result
	}
	start, end := 1, k
	if k < 0 {
		start, end = n+k, n-1
	}
	window := 0
	for j := start; j <= end; j++ {
		window += code[j%n]
	}
	for i := 0; i < n; i++ {
		result[i] = window
		window -= code[start%n]
		start++
		end++
		window += code[end%n]
	}
	return result
}
