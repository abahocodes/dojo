package main

func pathSum(root *TreeNode, targetSum int) [][]int {
	var result [][]int
	if root == nil {
		return result
	}
	type frame struct {
		node    *TreeNode
		leaving bool // undo this node on the way back up
	}
	var path []int
	total := 0
	stack := []frame{{root, false}}
	for len(stack) > 0 {
		f := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		node := f.node
		if f.leaving {
			path = path[:len(path)-1]
			total -= node.Val
			continue
		}
		path = append(path, node.Val)
		total += node.Val
		stack = append(stack, frame{node, true})
		if node.Left == nil && node.Right == nil {
			if total == targetSum {
				result = append(result, append([]int(nil), path...))
			}
		} else {
			// push right first so the left subtree is explored first
			if node.Right != nil {
				stack = append(stack, frame{node.Right, false})
			}
			if node.Left != nil {
				stack = append(stack, frame{node.Left, false})
			}
		}
	}
	return result
}
