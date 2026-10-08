package main

import "math"

func isEvenOddTree(root *TreeNode) bool {
	level := []*TreeNode{root}
	for depth := 0; len(level) > 0; depth++ {
		evenLevel := depth%2 == 0
		// Sentinel just outside the value range [1, 10^6].
		prev := 0
		if !evenLevel {
			prev = math.MaxInt
		}
		var next []*TreeNode
		for _, node := range level {
			v := node.Val
			if evenLevel {
				if v%2 == 0 || v <= prev {
					return false
				}
			} else {
				if v%2 == 1 || v >= prev {
					return false
				}
			}
			prev = v
			if node.Left != nil {
				next = append(next, node.Left)
			}
			if node.Right != nil {
				next = append(next, node.Right)
			}
		}
		level = next
	}
	return true
}
