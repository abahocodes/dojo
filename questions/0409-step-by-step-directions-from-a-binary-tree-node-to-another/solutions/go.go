package main

import "strings"

func getDirections(root *TreeNode, startValue int, destValue int) string {
	parent := map[int]int{}
	move := map[int]byte{}
	stack := []*TreeNode{root}
	for len(stack) > 0 {
		node := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		if node.Left != nil {
			parent[node.Left.Val] = node.Val
			move[node.Left.Val] = 'L'
			stack = append(stack, node.Left)
		}
		if node.Right != nil {
			parent[node.Right.Val] = node.Val
			move[node.Right.Val] = 'R'
			stack = append(stack, node.Right)
		}
	}
	pathFromRoot := func(value int) []byte {
		var moves []byte
		for {
			p, ok := parent[value]
			if !ok {
				break
			}
			moves = append(moves, move[value])
			value = p
		}
		for i, j := 0, len(moves)-1; i < j; i, j = i+1, j-1 {
			moves[i], moves[j] = moves[j], moves[i]
		}
		return moves
	}
	toStart := pathFromRoot(startValue)
	toDest := pathFromRoot(destValue)
	common := 0
	for common < len(toStart) && common < len(toDest) && toStart[common] == toDest[common] {
		common++
	}
	return strings.Repeat("U", len(toStart)-common) + string(toDest[common:])
}
