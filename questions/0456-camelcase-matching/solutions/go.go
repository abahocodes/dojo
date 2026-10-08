package main

func camelMatch(queries []string, pattern string) []bool {
	result := make([]bool, len(queries))
	for k, q := range queries {
		j := 0
		ok := true
		for i := 0; i < len(q); i++ {
			c := q[i]
			if j < len(pattern) && c == pattern[j] {
				j++
			} else if c >= 'A' && c <= 'Z' {
				ok = false
				break
			}
		}
		result[k] = ok && j == len(pattern)
	}
	return result
}
