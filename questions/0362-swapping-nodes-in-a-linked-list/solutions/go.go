package main

func swapNodes(head *ListNode, k int) *ListNode {
	first := head
	for i := 1; i < k; i++ {
		first = first.Next
	}
	runner, second := first, head
	for runner.Next != nil {
		runner = runner.Next
		second = second.Next
	}
	first.Val, second.Val = second.Val, first.Val
	return head
}
