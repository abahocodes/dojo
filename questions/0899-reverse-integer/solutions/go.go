package main

import "math"

func reverseInteger(x int) int {
	v := int32(x)
	var rev int32
	for v != 0 {
		digit := v % 10
		v /= 10
		if rev > math.MaxInt32/10 || (rev == math.MaxInt32/10 && digit > 7) {
			return 0
		}
		if rev < math.MinInt32/10 || (rev == math.MinInt32/10 && digit < -8) {
			return 0
		}
		rev = rev*10 + digit
	}
	return int(rev)
}
