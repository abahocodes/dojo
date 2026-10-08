package main

func longestSquareStreak(nums []int) int {
	mx := 0
	for _, x := range nums {
		if x > mx {
			mx = x
		}
	}
	present := make([]bool, mx+1)
	for _, x := range nums {
		present[x] = true
	}
	best := -1
	for x := 2; x <= mx; x++ {
		if !present[x] {
			continue
		}
		length := 1
		cur := x
		for cur*cur <= mx && present[cur*cur] {
			cur *= cur
			length++
		}
		if length >= 2 && length > best {
			best = length
		}
	}
	return best
}
