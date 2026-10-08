package main

func numberOfNiceSubarrays(nums []int, k int) int {
	seen := make([]int, len(nums)+1)
	seen[0] = 1
	odds, total := 0, 0
	for _, x := range nums {
		odds += x & 1
		if odds >= k {
			total += seen[odds-k]
		}
		seen[odds]++
	}
	return total
}
