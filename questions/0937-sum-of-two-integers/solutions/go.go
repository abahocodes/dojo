package main

func getSum(a int, b int) int {
	x, y := int32(a), int32(b)
	for y != 0 {
		carry := (x & y) << 1
		x = x ^ y
		y = carry
	}
	return int(x)
}
