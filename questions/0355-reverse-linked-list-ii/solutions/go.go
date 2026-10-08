package main

func reverseBetween(head *ListNode, left int, right int) *ListNode {
	dummy := &ListNode{Next: head}
	before := dummy
	for i := 0; i < left-1; i++ {
		before = before.Next
	}
	tail := before.Next
	for i := 0; i < right-left; i++ {
		move := tail.Next
		tail.Next = move.Next
		move.Next = before.Next
		before.Next = move
	}
	return dummy.Next
}
