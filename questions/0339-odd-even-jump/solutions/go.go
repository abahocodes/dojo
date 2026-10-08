package main

import "sort"

func oddEvenJumps(arr []int) int {
	n := len(arr)
	// For each index, the first later entry of `order` that lies to its right.
	targets := func(order []int) []int {
		nxt := make([]int, n)
		for i := range nxt {
			nxt[i] = -1
		}
		stack := make([]int, 0, n)
		for _, j := range order {
			for len(stack) > 0 && stack[len(stack)-1] < j {
				nxt[stack[len(stack)-1]] = j
				stack = stack[:len(stack)-1]
			}
			stack = append(stack, j)
		}
		return nxt
	}
	byAsc := make([]int, n)
	byDesc := make([]int, n)
	for i := 0; i < n; i++ {
		byAsc[i] = i
		byDesc[i] = i
	}
	sort.Slice(byAsc, func(x, y int) bool {
		a, b := byAsc[x], byAsc[y]
		if arr[a] != arr[b] {
			return arr[a] < arr[b]
		}
		return a < b
	})
	sort.Slice(byDesc, func(x, y int) bool {
		a, b := byDesc[x], byDesc[y]
		if arr[a] != arr[b] {
			return arr[a] > arr[b]
		}
		return a < b
	})
	oddNext := targets(byAsc)
	evenNext := targets(byDesc)
	odd := make([]bool, n)
	even := make([]bool, n)
	odd[n-1], even[n-1] = true, true
	count := 1
	for i := n - 2; i >= 0; i-- {
		if oddNext[i] != -1 {
			odd[i] = even[oddNext[i]]
		}
		if evenNext[i] != -1 {
			even[i] = odd[evenNext[i]]
		}
		if odd[i] {
			count++
		}
	}
	return count
}
