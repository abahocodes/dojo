package main

func subarraysWithKDistinct(nums []int, k int) int {
	atMost := func(limit int) int {
		count := make([]int, len(nums)+1)
		distinct, left, total := 0, 0, 0
		for right, v := range nums {
			if count[v] == 0 {
				distinct++
			}
			count[v]++
			for distinct > limit {
				count[nums[left]]--
				if count[nums[left]] == 0 {
					distinct--
				}
				left++
			}
			total += right - left + 1
		}
		return total
	}
	return atMost(k) - atMost(k-1)
}
