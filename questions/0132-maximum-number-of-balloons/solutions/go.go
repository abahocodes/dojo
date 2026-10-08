package main

func maxNumberOfBalloons(text string) int {
	var have, need [26]int
	for i := 0; i < len(text); i++ {
		have[text[i]-'a']++
	}
	for _, c := range "balloon" {
		need[c-'a']++
	}
	best := len(text)
	for i := 0; i < 26; i++ {
		if need[i] > 0 && have[i]/need[i] < best {
			best = have[i] / need[i]
		}
	}
	return best
}
