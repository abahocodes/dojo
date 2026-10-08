function rotateRight(head, k) {
  if (head === null) return null;
  let n = 1;
  let tail = head;
  while (tail.next !== null) {
    tail = tail.next;
    n++;
  }
  const r = k % n;
  if (r === 0) return head;
  tail.next = head;
  let newTail = head;
  for (let i = 0; i < n - r - 1; i++) newTail = newTail.next;
  const newHead = newTail.next;
  newTail.next = null;
  return newHead;
}
