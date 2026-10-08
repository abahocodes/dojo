package main

func maxMinPower(stations []int, r int, k int) int {
	n := len(stations)
	// power[i] = sum of stations in [i - r, i + r], via a sliding window.
	power := make([]int, n)
	window := 0
	for i := 0; i < n && i <= r; i++ {
		window += stations[i]
	}
	for i := 0; i < n; i++ {
		power[i] = window
		if i+r+1 < n {
			window += stations[i+r+1]
		}
		if i-r >= 0 {
			window -= stations[i-r]
		}
	}
	lo := power[0]
	for _, p := range power {
		if p < lo {
			lo = p
		}
	}
	hi := lo + k
	added := make([]int, n+1)
	feasible := func(target int) bool {
		for i := range added {
			added[i] = 0
		}
		extra, used := 0, 0
		for i := 0; i < n; i++ {
			extra += added[i]
			have := power[i] + extra
			if have < target {
				need := target - have
				used += need
				if used > k {
					return false
				}
				extra += need
				end := i + 2*r + 1
				if end > n {
					end = n
				}
				added[end] -= need
			}
		}
		return true
	}
	for lo < hi {
		mid := lo + (hi-lo+1)/2
		if feasible(mid) {
			lo = mid
		} else {
			hi = mid - 1
		}
	}
	return lo
}
