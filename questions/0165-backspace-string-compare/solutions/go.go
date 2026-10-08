package main

// prevChar returns the index of the next surviving byte at or before i, or -1.
func prevChar(text string, i int) int {
	skip := 0
	for i >= 0 {
		if text[i] == '#' {
			skip++
		} else if skip > 0 {
			skip--
		} else {
			return i
		}
		i--
	}
	return -1
}

func backspaceCompare(s string, t string) bool {
	i, j := len(s)-1, len(t)-1
	for {
		i = prevChar(s, i)
		j = prevChar(t, j)
		if i < 0 || j < 0 {
			return i < 0 && j < 0
		}
		if s[i] != t[j] {
			return false
		}
		i--
		j--
	}
}
