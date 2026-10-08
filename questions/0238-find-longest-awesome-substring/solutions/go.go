package main

func longestAwesome(s string) int {
	n := len(s)
	var first [1024]int
	for m := range first {
		first[m] = n + 1
	}
	first[0] = 0
	mask, best := 0, 0
	for i := 1; i <= n; i++ {
		mask ^= 1 << (s[i-1] - '0')
		if i-first[mask] > best {
			best = i - first[mask]
		}
		for d := 0; d < 10; d++ {
			if c := i - first[mask^(1<<d)]; c > best {
				best = c
			}
		}
		if first[mask] > i {
			first[mask] = i
		}
	}
	return best
}
