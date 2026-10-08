package main

import "math"

func stringToInteger(s string) int {
	n := len(s)
	i := 0
	for i < n && s[i] == ' ' {
		i++
	}
	sign := 1
	if i < n && (s[i] == '+' || s[i] == '-') {
		if s[i] == '-' {
			sign = -1
		}
		i++
	}
	value := 0
	for i < n && s[i] >= '0' && s[i] <= '9' {
		value = value*10 + int(s[i]-'0')
		if value > math.MaxInt32 { // stop early: the result is clamped anyway
			if sign == 1 {
				return math.MaxInt32
			}
			return math.MinInt32
		}
		i++
	}
	return sign * value
}
