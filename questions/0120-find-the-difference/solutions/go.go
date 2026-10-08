package main

func findTheDifference(s string, t string) string {
	var x byte
	for i := 0; i < len(s); i++ {
		x ^= s[i]
	}
	for i := 0; i < len(t); i++ {
		x ^= t[i]
	}
	return string([]byte{x})
}
