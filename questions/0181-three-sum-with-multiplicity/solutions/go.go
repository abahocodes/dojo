package main

func threeSumMulti(arr []int, target int) int {
	const mod = 1_000_000_007
	var c [101]int64
	for _, v := range arr {
		c[v]++
	}
	var total int64
	for x := 0; x <= 100; x++ {
		for y := x; y <= 100; y++ {
			z := target - x - y
			if z < y || z > 100 {
				continue
			}
			var ways int64
			switch {
			case x == y && y == z:
				ways = c[x] * (c[x] - 1) * (c[x] - 2) / 6
			case x == y:
				ways = c[x] * (c[x] - 1) / 2 * c[z]
			case y == z:
				ways = c[x] * (c[y] * (c[y] - 1) / 2)
			default:
				ways = c[x] * c[y] * c[z]
			}
			total = (total + ways%mod) % mod
		}
	}
	return int(total)
}
