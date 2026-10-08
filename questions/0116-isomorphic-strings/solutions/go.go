package main

func isIsomorphic(s string, t string) bool {
	// forward[a] / backward[b] hold the partner byte + 1 (0 = unmapped).
	var forward, backward [128]int
	for i := 0; i < len(s); i++ {
		a, b := int(s[i]), int(t[i])
		if forward[a] == 0 && backward[b] == 0 {
			forward[a] = b + 1
			backward[b] = a + 1
		} else if forward[a] != b+1 {
			return false
		}
	}
	return true
}
