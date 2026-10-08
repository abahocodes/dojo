package main

import "math"

func closestValue(root *TreeNode, target float64) int {
	best := root.Val
	node := root
	for node != nil {
		v := node.Val
		d := math.Abs(float64(v) - target)
		bd := math.Abs(float64(best) - target)
		if d < bd || (d == bd && v < best) {
			best = v
		}
		if target < float64(v) {
			node = node.Left
		} else if target > float64(v) {
			node = node.Right
		} else {
			break
		}
	}
	return best
}
