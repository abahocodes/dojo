package main

func trap(height []int) int {
	lo, hi := 0, len(height)-1
	leftMax, rightMax := 0, 0
	water := 0
	for lo <= hi {
		if leftMax <= rightMax {
			leftMax = max(leftMax, height[lo])
			water += leftMax - height[lo]
			lo++
		} else {
			rightMax = max(rightMax, height[hi])
			water += rightMax - height[hi]
			hi--
		}
	}
	return water
}
