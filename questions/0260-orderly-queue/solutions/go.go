package main

import "sort"

func orderlyQueue(s string, k int) string {
	if k == 1 {
		doubled := s + s
		best := s
		for i := 1; i < len(s); i++ {
			if cand := doubled[i : i+len(s)]; cand < best {
				best = cand
			}
		}
		return best
	}
	b := []byte(s)
	sort.Slice(b, func(i, j int) bool { return b[i] < b[j] })
	return string(b)
}
