package main

func containsNearbyDuplicate(nums []int, k int) bool {
	last := make(map[int]int)
	for i, x := range nums {
		if j, ok := last[x]; ok && i-j <= k {
			return true
		}
		last[x] = i
	}
	return false
}
