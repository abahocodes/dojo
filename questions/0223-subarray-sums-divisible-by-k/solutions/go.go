package main

func subarraysDivByK(nums []int, k int) int {
	count := make([]int, k)
	count[0] = 1
	rem, result := 0, 0
	for _, x := range nums {
		rem = ((rem+x)%k + k) % k
		result += count[rem]
		count[rem]++
	}
	return result
}
