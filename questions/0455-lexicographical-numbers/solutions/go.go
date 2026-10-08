package main

func lexicalOrder(n int) []int {
	result := make([]int, 0, n)
	cur := 1
	for i := 0; i < n; i++ {
		result = append(result, cur)
		if cur*10 <= n {
			cur *= 10
		} else {
			for cur%10 == 9 || cur+1 > n {
				cur /= 10
			}
			cur++
		}
	}
	return result
}
