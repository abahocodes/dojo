package main

func reorderList(head *ListNode) *ListNode {
	if head == nil || head.Next == nil {
		return head
	}
	slow, fast := head, head
	for fast.Next != nil && fast.Next.Next != nil {
		slow = slow.Next
		fast = fast.Next.Next
	}
	second := slow.Next
	slow.Next = nil
	var prev *ListNode
	for second != nil {
		nxt := second.Next
		second.Next = prev
		prev = second
		second = nxt
	}
	first := head
	second = prev
	for second != nil {
		n1, n2 := first.Next, second.Next
		first.Next = second
		second.Next = n1
		first, second = n1, n2
	}
	return head
}
