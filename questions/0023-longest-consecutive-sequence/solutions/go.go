package main

func longestConsecutiveSequence(nums []int) int {
	values := make(map[int]struct{}, len(nums))
	for _, x := range nums {
		values[x] = struct{}{}
	}
	best := 0
	for x := range values {
		if _, ok := values[x-1]; !ok {
			length := 1
			for {
				if _, ok := values[x+length]; !ok {
					break
				}
				length++
			}
			best = max(best, length)
		}
	}
	return best
}
