package main

func countBinarySubstrings(s string) int {
	total, prevRun, curRun := 0, 0, 1
	for i := 1; i < len(s); i++ {
		if s[i] == s[i-1] {
			curRun++
		} else {
			total += min(prevRun, curRun)
			prevRun, curRun = curRun, 1
		}
	}
	return total + min(prevRun, curRun)
}
