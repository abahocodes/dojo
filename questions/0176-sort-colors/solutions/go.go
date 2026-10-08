package main

func sortColors(nums []int) []int {
	a := append([]int(nil), nums...)
	low, mid, high := 0, 0, len(a)-1
	for mid <= high {
		switch a[mid] {
		case 0:
			a[low], a[mid] = a[mid], a[low]
			low++
			mid++
		case 1:
			mid++
		default:
			a[mid], a[high] = a[high], a[mid]
			high--
		}
	}
	return a
}
