package main

func mergeSorted(nums1 []int, nums2 []int) []int {
	m, n := len(nums1), len(nums2)
	out := make([]int, m+n)
	copy(out, nums1)
	i, j, w := m-1, n-1, m+n-1
	for j >= 0 {
		if i >= 0 && out[i] > nums2[j] {
			out[w] = out[i]
			i--
		} else {
			out[w] = nums2[j]
			j--
		}
		w--
	}
	return out
}
