package main

func commonChars(words []string) []string {
	var common [26]int
	for i := range common {
		common[i] = 1 << 30
	}
	for _, w := range words {
		var freq [26]int
		for i := 0; i < len(w); i++ {
			freq[w[i]-'a']++
		}
		for i := 0; i < 26; i++ {
			if freq[i] < common[i] {
				common[i] = freq[i]
			}
		}
	}
	result := []string{}
	for i := 0; i < 26; i++ {
		for j := 0; j < common[i]; j++ {
			result = append(result, string(rune('a'+i)))
		}
	}
	return result
}
