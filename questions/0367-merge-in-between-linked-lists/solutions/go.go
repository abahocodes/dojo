package main

func mergeInBetween(list1 *ListNode, a int, b int, list2 *ListNode) *ListNode {
	before := list1
	for i := 0; i < a-1; i++ {
		before = before.Next
	}
	after := before
	for i := 0; i < b-a+2; i++ {
		after = after.Next
	}
	before.Next = list2
	tail := list2
	for tail.Next != nil {
		tail = tail.Next
	}
	tail.Next = after
	return list1
}
