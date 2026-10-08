package main

func canJump(nums []int) bool {
	furthest := 0
	last := len(nums) - 1
	for i, step := range nums {
		if i > furthest {
			return false
		}
		furthest = max(furthest, i+step)
		if furthest >= last {
			return true
		}
	}
	return true
}
