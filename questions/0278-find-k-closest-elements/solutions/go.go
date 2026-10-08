package main

func findClosestElements(arr []int, k int, x int) []int {
	// Binary search for the left edge of the best window arr[left .. left+k-1].
	lo, hi := 0, len(arr)-k
	for lo < hi {
		mid := (lo + hi) / 2
		if x-arr[mid] > arr[mid+k]-x {
			lo = mid + 1
		} else {
			hi = mid
		}
	}
	out := make([]int, k)
	copy(out, arr[lo:lo+k])
	return out
}
