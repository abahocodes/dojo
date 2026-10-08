package main

func maxProduct(nums []int) int {
	best, hi, lo := nums[0], nums[0], nums[0]
	for _, x := range nums[1:] {
		if x < 0 {
			hi, lo = lo, hi
		}
		hi = max(x, hi*x)
		lo = min(x, lo*x)
		best = max(best, hi)
	}
	return best
}
