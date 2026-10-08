package main

func countSubstrings(s string) int {
	n := len(s)
	count := 0
	// Every palindrome has a center: a character (odd length) or a gap (even length).
	for center := 0; center < 2*n-1; center++ {
		left := center / 2
		right := left + center%2
		for left >= 0 && right < n && s[left] == s[right] {
			count++
			left--
			right++
		}
	}
	return count
}
