package main

func intersect(nums1 []int, nums2 []int) []int {
	var counts [1001]int
	for _, x := range nums1 {
		counts[x]++
	}
	out := []int{}
	for _, x := range nums2 {
		if counts[x] > 0 {
			counts[x]--
			out = append(out, x)
		}
	}
	return out
}
