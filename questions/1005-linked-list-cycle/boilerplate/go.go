// 1005. Linked List Cycle (easy)
// /show to reread the problem · /test runs visible tests · /submit when done
package main

// Provided by dojo:
// type ListNode struct {
//     Val  int
//     Next *ListNode
// }

func hasCycle(head *ListNode) bool {
	return false
}

// dojo calls this to run the tests; you don't need to change it.
func runHasCycle(values []int, pos int) bool {
	nodes := make([]*ListNode, len(values))
	for i, v := range values {
		nodes[i] = &ListNode{Val: v}
	}
	for i := 0; i+1 < len(nodes); i++ {
		nodes[i].Next = nodes[i+1]
	}
	if pos >= 0 {
		nodes[len(nodes)-1].Next = nodes[pos]
	}
	var head *ListNode
	if len(nodes) > 0 {
		head = nodes[0]
	}
	return hasCycle(head)
}
