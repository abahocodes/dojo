package main

func canConstruct(ransomNote string, magazine string) bool {
	if len(ransomNote) > len(magazine) {
		return false
	}
	var counts [26]int
	for i := 0; i < len(magazine); i++ {
		counts[magazine[i]-'a']++
	}
	for i := 0; i < len(ransomNote); i++ {
		k := ransomNote[i] - 'a'
		counts[k]--
		if counts[k] < 0 {
			return false
		}
	}
	return true
}
