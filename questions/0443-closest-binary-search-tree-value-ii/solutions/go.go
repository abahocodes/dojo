package main

func closestKValues(root *TreeNode, target float64, k int) []int {
	// pred: path to values <= target (top = largest); succ: values > target (top = smallest).
	var pred, succ []*TreeNode
	for node := root; node != nil; {
		if float64(node.Val) <= target {
			pred = append(pred, node)
			node = node.Right
		} else {
			succ = append(succ, node)
			node = node.Left
		}
	}
	result := make([]int, 0, k)
	for i := 0; i < k; i++ {
		takeSmaller := len(succ) == 0 ||
			(len(pred) > 0 && target-float64(pred[len(pred)-1].Val) <= float64(succ[len(succ)-1].Val)-target)
		if takeSmaller {
			top := pred[len(pred)-1]
			pred = pred[:len(pred)-1]
			for cur := top.Left; cur != nil; cur = cur.Right {
				pred = append(pred, cur)
			}
			result = append(result, top.Val)
		} else {
			top := succ[len(succ)-1]
			succ = succ[:len(succ)-1]
			for cur := top.Right; cur != nil; cur = cur.Left {
				succ = append(succ, cur)
			}
			result = append(result, top.Val)
		}
	}
	return result
}
