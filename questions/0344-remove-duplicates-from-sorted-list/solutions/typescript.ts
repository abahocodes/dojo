function deleteDuplicates(head: ListNode | null): ListNode | null {
  let cur = head;
  while (cur !== null && cur.next !== null) {
    if (cur.next.val === cur.val) {
      cur.next = cur.next.next;
    } else {
      cur = cur.next;
    }
  }
  return head;
}
