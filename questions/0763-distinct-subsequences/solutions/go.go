package main

func numDistinct(s string, t string) int {
	m := len(t)
	// ways[j] = number of ways to form t[:j] from the part of s read so far.
	// Counts for short prefixes of t can overflow; uint64 arithmetic wraps
	// modulo 2^64, and since the true answer fits in an int32, ways[m] is exact.
	ways := make([]uint64, m+1)
	ways[0] = 1
	for i := 0; i < len(s); i++ {
		// Go right to left so ways[j-1] still holds the previous row's value.
		for j := m; j >= 1; j-- {
			if t[j-1] == s[i] {
				ways[j] += ways[j-1]
			}
		}
	}
	return int(ways[m])
}
