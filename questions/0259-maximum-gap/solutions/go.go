package main

func maximumGap(nums []int) int {
	n := len(nums)
	if n < 2 {
		return 0
	}
	lo, hi := nums[0], nums[0]
	for _, v := range nums {
		lo = min(lo, v)
		hi = max(hi, v)
	}
	if lo == hi {
		return 0
	}
	size := max(1, (hi-lo)/(n-1))
	count := (hi-lo)/size + 1
	bucketMin := make([]int, count)
	bucketMax := make([]int, count)
	for b := range bucketMax {
		bucketMax[b] = -1
	}
	for _, v := range nums {
		b := (v - lo) / size
		if bucketMax[b] < 0 || v < bucketMin[b] {
			bucketMin[b] = v
		}
		bucketMax[b] = max(bucketMax[b], v)
	}
	best, prev := 0, lo
	for b := 0; b < count; b++ {
		if bucketMax[b] < 0 {
			continue // empty
		}
		best = max(best, bucketMin[b]-prev)
		prev = bucketMax[b]
	}
	return best
}
