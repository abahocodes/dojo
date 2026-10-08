package main

func rotateRight(head *ListNode, k int) *ListNode {
	if head == nil {
		return nil
	}
	n := 1
	tail := head
	for tail.Next != nil {
		tail = tail.Next
		n++
	}
	r := k % n
	if r == 0 {
		return head
	}
	tail.Next = head
	newTail := head
	for i := 0; i < n-r-1; i++ {
		newTail = newTail.Next
	}
	newHead := newTail.Next
	newTail.Next = nil
	return newHead
}
