package main

func longestMountain(arr []int) int {
	best, up, down := 0, 0, 0
	for i := 1; i < len(arr); i++ {
		if arr[i-1] == arr[i] || (down > 0 && arr[i-1] < arr[i]) {
			up, down = 0, 0
		}
		if arr[i-1] < arr[i] {
			up++
		} else if arr[i-1] > arr[i] {
			down++
		}
		if up > 0 && down > 0 && up+down+1 > best {
			best = up + down + 1
		}
	}
	return best
}
