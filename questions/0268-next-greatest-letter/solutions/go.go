package main

func nextGreatestLetter(letters []string, target string) string {
	lo, hi := 0, len(letters)
	for lo < hi {
		mid := (lo + hi) / 2
		if letters[mid] <= target {
			lo = mid + 1
		} else {
			hi = mid
		}
	}
	return letters[lo%len(letters)]
}
