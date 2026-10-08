package main

func countSubarraysMaxK(nums []int, k int) int {
	m := 0
	for _, v := range nums {
		if v > m {
			m = v
		}
	}
	count, left, total := 0, 0, 0
	for _, v := range nums {
		if v == m {
			count++
		}
		for count >= k {
			if nums[left] == m {
				count--
			}
			left++
		}
		total += left
	}
	return total
}
