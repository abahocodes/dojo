package main

func findLhs(nums []int) int {
	count := make(map[int]int)
	for _, x := range nums {
		count[x]++
	}
	best := 0
	for x, c := range count {
		if next, ok := count[x+1]; ok && c+next > best {
			best = c + next
		}
	}
	return best
}
