package main

import (
	"strconv"
	"strings"
)

func deserialize(data string) *TreeNode {
	tokens := strings.Split(data, ",")
	if tokens[0] == "#" {
		return nil
	}
	parse := func(tok string) *TreeNode {
		if tok == "#" {
			return nil
		}
		v, _ := strconv.Atoi(tok)
		return &TreeNode{Val: v}
	}
	root := parse(tokens[0])
	stack := []*TreeNode{root}
	leftDone := []bool{false}
	for _, tok := range tokens[1:] {
		child := parse(tok)
		top := len(stack) - 1
		if !leftDone[top] {
			stack[top].Left = child
			leftDone[top] = true
		} else {
			stack[top].Right = child
			stack = stack[:top]
			leftDone = leftDone[:top]
		}
		if child != nil {
			stack = append(stack, child)
			leftDone = append(leftDone, false)
		}
	}
	return root
}
