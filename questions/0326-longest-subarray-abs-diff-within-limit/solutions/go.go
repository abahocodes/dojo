package main

func longestSubarrayLimit(nums []int, limit int) int {
	// Slices used as deques: append at the back, advance a head for the front.
	maxq := make([]int, 0, len(nums))
	minq := make([]int, 0, len(nums))
	maxHead, minHead := 0, 0
	left, best := 0, 0
	for right, x := range nums {
		for len(maxq) > maxHead && nums[maxq[len(maxq)-1]] < x {
			maxq = maxq[:len(maxq)-1]
		}
		maxq = append(maxq, right)
		for len(minq) > minHead && nums[minq[len(minq)-1]] > x {
			minq = minq[:len(minq)-1]
		}
		minq = append(minq, right)
		for nums[maxq[maxHead]]-nums[minq[minHead]] > limit {
			left++
			if maxq[maxHead] < left {
				maxHead++
			}
			if minq[minHead] < left {
				minHead++
			}
		}
		if right-left+1 > best {
			best = right - left + 1
		}
	}
	return best
}
