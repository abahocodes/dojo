package main

func findLengthOfShortestSubarray(arr []int) int {
	n := len(arr)
	right := n - 1
	for right > 0 && arr[right-1] <= arr[right] {
		right--
	}
	if right == 0 {
		return 0
	}
	best := right
	for left := 0; left < n; left++ {
		if left > 0 && arr[left-1] > arr[left] {
			break
		}
		for right < n && arr[right] < arr[left] {
			right++
		}
		if right-left-1 < best {
			best = right - left - 1
		}
	}
	return best
}
