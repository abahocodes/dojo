package main

func groupAnagrams(words []string) [][]string {
	index := make(map[[26]int]int)
	var result [][]string
	for _, w := range words {
		var counts [26]int
		for i := 0; i < len(w); i++ {
			counts[w[i]-'a']++
		}
		if idx, ok := index[counts]; ok {
			result[idx] = append(result[idx], w)
		} else {
			index[counts] = len(result)
			result = append(result, []string{w})
		}
	}
	return result
}
