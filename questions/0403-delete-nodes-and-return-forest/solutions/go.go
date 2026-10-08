package main

func delNodes(root *TreeNode, toDelete []int) []*TreeNode {
	doomed := make(map[int]bool, len(toDelete))
	for _, v := range toDelete {
		doomed[v] = true
	}
	var forest []*TreeNode
	// Each entry: the node, and whether it is the top of a tree once its parent is gone.
	type frame struct {
		node   *TreeNode
		isRoot bool
	}
	stack := []frame{{root, true}}
	for len(stack) > 0 {
		f := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		node := f.node
		deleted := doomed[node.Val]
		if f.isRoot && !deleted {
			forest = append(forest, node)
		}
		if node.Left != nil {
			stack = append(stack, frame{node.Left, deleted})
			if doomed[node.Left.Val] {
				node.Left = nil
			}
		}
		if node.Right != nil {
			stack = append(stack, frame{node.Right, deleted})
			if doomed[node.Right.Val] {
				node.Right = nil
			}
		}
	}
	return forest
}
