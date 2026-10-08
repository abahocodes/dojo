package main

func maxSubArrayLen(nums []int, k int) int {
	first := map[int]int{0: -1}
	prefix, best := 0, 0
	for i, x := range nums {
		prefix += x
		if j, ok := first[prefix-k]; ok && i-j > best {
			best = i - j
		}
		if _, ok := first[prefix]; !ok {
			first[prefix] = i
		}
	}
	return best
}
