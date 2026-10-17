package main

func myPow(x float64, n int) float64 {
	// Binary exponentiation: square the base and halve the exponent,
	// multiplying the base into the result whenever the low bit is set.
	e := int64(n)
	if e < 0 {
		x = 1 / x
		e = -e
	}
	result := 1.0
	for e > 0 {
		if e&1 == 1 {
			result *= x
		}
		x *= x
		e >>= 1
	}
	return result
}
