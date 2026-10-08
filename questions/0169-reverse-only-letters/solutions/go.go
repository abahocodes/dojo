package main

func reverseOnlyLetters(s string) string {
	isLetter := func(c byte) bool { return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') }
	chars := []byte(s)
	lo, hi := 0, len(chars)-1
	for lo < hi {
		if !isLetter(chars[lo]) {
			lo++
		} else if !isLetter(chars[hi]) {
			hi--
		} else {
			chars[lo], chars[hi] = chars[hi], chars[lo]
			lo++
			hi--
		}
	}
	return string(chars)
}
