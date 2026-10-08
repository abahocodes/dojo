package main

func xorQueries(arr []int, queries [][]int) []int {
	prefix := make([]int, len(arr)+1)
	for i, x := range arr {
		prefix[i+1] = prefix[i] ^ x
	}
	answers := make([]int, len(queries))
	for i, q := range queries {
		answers[i] = prefix[q[1]+1] ^ prefix[q[0]]
	}
	return answers
}
