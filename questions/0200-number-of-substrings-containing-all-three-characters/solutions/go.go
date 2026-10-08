package main

func numberOfSubstrings(s string) int {
	last := [3]int{-1, -1, -1}
	total := 0
	for i := 0; i < len(s); i++ {
		last[s[i]-'a'] = i
		m := last[0]
		if last[1] < m {
			m = last[1]
		}
		if last[2] < m {
			m = last[2]
		}
		total += m + 1
	}
	return total
}
