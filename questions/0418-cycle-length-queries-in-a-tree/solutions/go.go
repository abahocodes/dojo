package main

func cycleLengthQueries(n int, queries [][]int) []int {
	answer := make([]int, len(queries))
	for i, q := range queries {
		a, b := q[0], q[1]
		steps := 0
		for a != b {
			if a > b {
				a >>= 1
			} else {
				b >>= 1
			}
			steps++
		}
		answer[i] = steps + 1
	}
	return answer
}
