package main

func rangeSums(nums []int, queries [][]int) []int {
	prefix := make([]int, len(nums)+1)
	for i, x := range nums {
		prefix[i+1] = prefix[i] + x
	}
	out := make([]int, len(queries))
	for i, q := range queries {
		out[i] = prefix[q[1]+1] - prefix[q[0]]
	}
	return out
}
