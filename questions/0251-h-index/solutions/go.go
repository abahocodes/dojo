package main

func hIndex(citations []int) int {
	n := len(citations)
	buckets := make([]int, n+1)
	for _, c := range citations {
		if c > n {
			c = n
		}
		buckets[c]++
	}
	papers := 0
	for h := n; h >= 0; h-- {
		papers += buckets[h]
		if papers >= h {
			return h
		}
	}
	return 0
}
