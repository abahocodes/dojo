package main

func fourSumCount(nums1 []int, nums2 []int, nums3 []int, nums4 []int) int {
	sums := make(map[int]int, len(nums1)*len(nums2))
	for _, a := range nums1 {
		for _, b := range nums2 {
			sums[a+b]++
		}
	}
	count := 0
	for _, c := range nums3 {
		for _, d := range nums4 {
			count += sums[-(c + d)]
		}
	}
	return count
}
