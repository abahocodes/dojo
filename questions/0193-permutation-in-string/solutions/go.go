package main

func checkInclusion(s1 string, s2 string) bool {
	m := len(s1)
	if m > len(s2) {
		return false
	}
	var need, have [26]int
	for j := 0; j < m; j++ {
		need[s1[j]-'a']++
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
	for j := 0; j < len(s2); j++ {
		change(int(s2[j]-'a'), 1)
		if j >= m {
			change(int(s2[j-m]-'a'), -1)
		}
		if j >= m-1 && matches == 26 {
			return true
		}
	}
	return false
}
