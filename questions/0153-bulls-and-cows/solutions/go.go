package main

import "strconv"

func getHint(secret string, guess string) string {
	bulls, cows := 0, 0
	var bal [10]int
	for i := 0; i < len(secret); i++ {
		a, b := int(secret[i]-'0'), int(guess[i]-'0')
		if a == b {
			bulls++
			continue
		}
		if bal[a] < 0 {
			cows++
		}
		if bal[b] > 0 {
			cows++
		}
		bal[a]++
		bal[b]--
	}
	return strconv.Itoa(bulls) + "A" + strconv.Itoa(cows) + "B"
}
