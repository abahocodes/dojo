package main

func minSwapsCircular(nums []int) int {
	n := len(nums)
	ones := 0
	for _, v := range nums {
		ones += v
	}
	if ones == 0 {
		return 0
	}
	window := 0
	for i := 0; i < ones; i++ {
		window += nums[i]
	}
	best := window
	for i := ones; i < ones+n-1; i++ {
		window += nums[i%n] - nums[i-ones]
		if window > best {
			best = window
		}
	}
	return ones - best
}
