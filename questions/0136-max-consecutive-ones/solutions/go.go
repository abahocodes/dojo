package main

func findMaxConsecutiveOnes(nums []int) int {
	best, run := 0, 0
	for _, x := range nums {
		if x == 1 {
			run++
			if run > best {
				best = run
			}
		} else {
			run = 0
		}
	}
	return best
}
