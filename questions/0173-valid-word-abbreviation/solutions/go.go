package main

func validWordAbbreviation(word string, abbr string) bool {
	n, m := len(word), len(abbr)
	i, j := 0, 0
	for i < n && j < m {
		c := abbr[j]
		if c >= '0' && c <= '9' {
			if c == '0' {
				return false
			}
			k := 0
			for j < m && abbr[j] >= '0' && abbr[j] <= '9' {
				k = k*10 + int(abbr[j]-'0')
				j++
			}
			i += k
		} else {
			if word[i] != c {
				return false
			}
			i++
			j++
		}
	}
	return i == n && j == m
}
