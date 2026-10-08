package main

func shiftingLetters(s string, shifts [][]int) string {
	n := len(s)
	diff := make([]int, n+1)
	for _, op := range shifts {
		delta := -1
		if op[2] == 1 {
			delta = 1
		}
		diff[op[0]] += delta
		diff[op[1]+1] -= delta
	}
	out := make([]byte, n)
	net := 0
	for i := 0; i < n; i++ {
		net += diff[i]
		code := ((int(s[i]-'a')+net)%26 + 26) % 26
		out[i] = byte('a' + code)
	}
	return string(out)
}
