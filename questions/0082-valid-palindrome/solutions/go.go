package main

func isPalindrome(s string) bool {
	keep := func(ch byte) bool {
		return (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') || (ch >= '0' && ch <= '9')
	}
	lower := func(ch byte) byte {
		if ch >= 'A' && ch <= 'Z' {
			return ch + ('a' - 'A')
		}
		return ch
	}
	left, right := 0, len(s)-1
	for left < right {
		if !keep(s[left]) {
			left++
		} else if !keep(s[right]) {
			right--
		} else if lower(s[left]) != lower(s[right]) {
			return false
		} else {
			left++
			right--
		}
	}
	return true
}
