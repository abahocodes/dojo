package main

func minKBitFlips(nums []int, k int) int {
	n := len(nums)
	ends := make([]int, n+1)
	active, flips := 0, 0
	for i := 0; i < n; i++ {
		active ^= ends[i]
		if nums[i]^active == 0 {
			if i+k > n {
				return -1
			}
			flips++
			active ^= 1
			ends[i+k] ^= 1
		}
	}
	return flips
}
