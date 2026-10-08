package main

func pathSumCount(root *TreeNode, targetSum int) int {
	if root == nil {
		return 0
	}
	seen := map[int]int{0: 1} // prefix sums on the current root-to-node path
	count := 0
	type frame struct {
		node    *TreeNode
		before  int
		leaving bool
	}
	stack := []frame{{root, 0, false}}
	for len(stack) > 0 {
		f := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		prefix := f.before + f.node.Val
		if f.leaving {
			seen[prefix]--
			continue
		}
		count += seen[prefix-targetSum]
		seen[prefix]++
		stack = append(stack, frame{f.node, f.before, true})
		if f.node.Right != nil {
			stack = append(stack, frame{f.node.Right, prefix, false})
		}
		if f.node.Left != nil {
			stack = append(stack, frame{f.node.Left, prefix, false})
		}
	}
	return count
}
