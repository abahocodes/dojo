package main

func nextLargerNodes(head *ListNode) []int {
	vals := []int{}
	for node := head; node != nil; node = node.Next {
		vals = append(vals, node.Val)
	}
	answer := make([]int, len(vals))
	waiting := make([]int, 0, len(vals))
	for i, v := range vals {
		for len(waiting) > 0 && vals[waiting[len(waiting)-1]] < v {
			answer[waiting[len(waiting)-1]] = v
			waiting = waiting[:len(waiting)-1]
		}
		waiting = append(waiting, i)
	}
	return answer
}
