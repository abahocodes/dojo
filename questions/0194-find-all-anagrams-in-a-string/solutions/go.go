package main

func findAnagrams(s string, p string) []int {
	m := len(p)
	result := []int{}
	if m > len(s) {
		return result
	}
	var need, have [26]int
	for j := 0; j < m; j++ {
		need[p[j]-'a']++
	}
	matches := 0
	for _, x := range need {
		if x == 0 {
			matches++
		}
	}
	change := func(i int, delta int) {
		if have[i] == need[i] {
			matches--
		}
		have[i] += delta
		if have[i] == need[i] {
			matches++
		}
	}
	for j := 0; j < len(s); j++ {
		change(int(s[j]-'a'), 1)
		if j >= m {
			change(int(s[j-m]-'a'), -1)
		}
		if j >= m-1 && matches == 26 {
			result = append(result, j-m+1)
		}
	}
	return result
}
