package main

func countSubarraysMedianK(nums []int, k int) int {
	n := len(nums)
	p := 0
	for nums[p] != k {
		p++
	}
	// right[b+off] counts right-side balances b.
	off := n + 1
	right := make([]int, 2*n+3)
	bal := 0
	right[off]++
	for i := p + 1; i < n; i++ {
		if nums[i] > k {
			bal++
		} else {
			bal--
		}
		right[bal+off]++
	}
	total := 0
	bal = 0
	for i := p; i >= 0; i-- {
		if i < p {
			if nums[i] > k {
				bal++
			} else {
				bal--
			}
		}
		total += right[-bal+off] + right[1-bal+off]
	}
	return total
}
