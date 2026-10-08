package main

func findShortestSubArray(nums []int) int {
	first := make(map[int]int)
	count := make(map[int]int)
	degree, best := 0, 0
	for i, x := range nums {
		if _, ok := first[x]; !ok {
			first[x] = i
		}
		count[x]++
		c := count[x]
		span := i - first[x] + 1
		if c > degree || (c == degree && span < best) {
			degree = c
			best = span
		}
	}
	return best
}
