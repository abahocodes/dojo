package main

func shortestSubarray(nums []int, k int) int {
	n := len(nums)
	prefix := make([]int, n+1)
	for i, x := range nums {
		prefix[i+1] = prefix[i] + x
	}
	// Deque of indices into prefix, stored in a slice with head/tail pointers.
	dq := make([]int, n+1)
	head, tail := 0, 0
	best := n + 1
	for j := 0; j <= n; j++ {
		for head < tail && prefix[j]-prefix[dq[head]] >= k {
			if j-dq[head] < best {
				best = j - dq[head]
			}
			head++
		}
		for head < tail && prefix[dq[tail-1]] >= prefix[j] {
			tail--
		}
		dq[tail] = j
		tail++
	}
	if best <= n {
		return best
	}
	return -1
}
