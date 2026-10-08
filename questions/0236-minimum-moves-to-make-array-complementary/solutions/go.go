package main

func minMovesComplementary(nums []int, limit int) int {
	n := len(nums)
	delta := make([]int, 2*limit+2)
	for i := 0; i < n/2; i++ {
		a, b := nums[i], nums[n-1-i]
		lo, hi := a, b
		if lo > hi {
			lo, hi = hi, lo
		}
		delta[2] += 2
		delta[lo+1]--
		delta[hi+limit+1]++
		delta[a+b]--
		delta[a+b+1]++
	}
	best, moves := n, 0
	for t := 2; t <= 2*limit; t++ {
		moves += delta[t]
		if moves < best {
			best = moves
		}
	}
	return best
}
