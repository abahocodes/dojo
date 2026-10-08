package main

func minSteps(s string, t string) int {
	var diff [26]int
	for i := 0; i < len(s); i++ {
		diff[s[i]-'a']++
	}
	for i := 0; i < len(t); i++ {
		diff[t[i]-'a']--
	}
	steps := 0
	for _, d := range diff {
		if d > 0 {
			steps += d
		}
	}
	return steps
}
