package main

func numOfWays(nums []int) int {
	const mod = 1_000_000_007
	n := len(nums)
	left := make([]int, n+1)
	right := make([]int, n+1)
	root := nums[0]
	for _, v := range nums[1:] {
		cur := root
		for {
			if v < cur {
				if left[cur] == 0 {
					left[cur] = v
					break
				}
				cur = left[cur]
			} else {
				if right[cur] == 0 {
					right[cur] = v
					break
				}
				cur = right[cur]
			}
		}
	}

	power := func(base, exp int) int {
		result := 1
		base %= mod
		for exp > 0 {
			if exp&1 == 1 {
				result = result * base % mod
			}
			base = base * base % mod
			exp >>= 1
		}
		return result
	}
	fact := make([]int, n+1)
	invFact := make([]int, n+1)
	fact[0] = 1
	for i := 1; i <= n; i++ {
		fact[i] = fact[i-1] * i % mod
	}
	invFact[n] = power(fact[n], mod-2)
	for i := n; i > 0; i-- {
		invFact[i-1] = invFact[i] * i % mod
	}

	size := make([]int, n+1)
	ways := make([]int, n+1)
	for i := range ways {
		ways[i] = 1
	}
	for i := n - 1; i >= 0; i-- {
		v := nums[i]
		l, r := left[v], right[v]
		size[v] = size[l] + size[r] + 1
		interleave := fact[size[l]+size[r]] * invFact[size[l]] % mod * invFact[size[r]] % mod
		ways[v] = interleave * ways[l] % mod * ways[r] % mod
	}
	return (ways[root] - 1 + mod) % mod
}
