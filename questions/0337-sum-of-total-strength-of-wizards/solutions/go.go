package main

func totalStrength(strength []int) int {
	const MOD = 1000000007
	n := len(strength)
	left := make([]int, n)
	right := make([]int, n)
	stack := make([]int, 0, n)
	for i := 0; i < n; i++ {
		for len(stack) > 0 && strength[stack[len(stack)-1]] >= strength[i] {
			right[stack[len(stack)-1]] = i // first smaller-or-equal to the right
			stack = stack[:len(stack)-1]
		}
		left[i] = -1 // first strictly smaller to the left
		if len(stack) > 0 {
			left[i] = stack[len(stack)-1]
		}
		stack = append(stack, i)
	}
	for _, j := range stack {
		right[j] = n
	}
	// pp[k] = P[0] + ... + P[k-1], where P[k] = strength[0] + ... + strength[k-1]
	pp := make([]int, n+2)
	p := 0
	for k := 0; k <= n; k++ {
		pp[k+1] = (pp[k] + p) % MOD
		if k < n {
			p = (p + strength[k]) % MOD
		}
	}
	total := 0
	for i := 0; i < n; i++ {
		l, r := left[i], right[i]
		plus := (i - l) * ((pp[r+1] - pp[i+1] + MOD) % MOD) % MOD
		minus := (r - i) * ((pp[i+1] - pp[l+1] + MOD) % MOD) % MOD
		span := (plus - minus + MOD) % MOD
		total = (total + strength[i]%MOD*span) % MOD
	}
	return total
}
