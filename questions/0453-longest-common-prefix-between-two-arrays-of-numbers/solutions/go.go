package main

func longestCommonPrefixNumbers(arr1 []int, arr2 []int) int {
	prefixes := make(map[int]bool)
	for _, x := range arr1 {
		for x > 0 && !prefixes[x] {
			prefixes[x] = true
			x /= 10
		}
	}
	best := 0
	for _, y := range arr2 {
		for y > 0 && !prefixes[y] {
			y /= 10
		}
		digits := 0
		for t := y; t > 0; t /= 10 {
			digits++
		}
		if digits > best {
			best = digits
		}
	}
	return best
}
