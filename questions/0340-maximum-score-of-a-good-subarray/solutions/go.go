package main

func maximumScore(nums []int, k int) int {
	n := len(nums)
	i, j := k, k
	low := nums[k]
	best := low
	for i > 0 || j < n-1 {
		if i == 0 || (j < n-1 && nums[j+1] > nums[i-1]) {
			j++
			if nums[j] < low {
				low = nums[j]
			}
		} else {
			i--
			if nums[i] < low {
				low = nums[i]
			}
		}
		if s := low * (j - i + 1); s > best {
			best = s
		}
	}
	return best
}
