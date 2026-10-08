package main

func minMovesKOnes(nums []int, k int) int {
	q := []int{}
	for i, x := range nums {
		if x == 1 {
			q = append(q, i-len(q))
		}
	}
	prefix := make([]int, len(q)+1)
	for i, v := range q {
		prefix[i+1] = prefix[i] + v
	}
	best := -1
	for lo := 0; lo+k <= len(q); lo++ {
		hi := lo + k - 1
		mid := lo + k/2
		m := q[mid]
		cost := m*(mid-lo) - (prefix[mid] - prefix[lo]) +
			(prefix[hi+1] - prefix[mid+1]) - m*(hi-mid)
		if best < 0 || cost < best {
			best = cost
		}
	}
	return best
}
