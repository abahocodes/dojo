// 1005. Linked List Cycle (easy)
// /show to reread the problem · /test runs visible tests · /submit when done

/**
 * Provided by dojo:
 * class ListNode {
 *     val: number
 *     next: ListNode | null
 *     constructor(val?: number, next?: ListNode | null)
 * }
 */

function hasCycle(head: ListNode | null): boolean {

}

// dojo calls this to run the tests; you don't need to change it.
function runHasCycle(values: number[], pos: number): boolean {
  const nodes = values.map((v) => new ListNode(v));
  for (let i = 0; i + 1 < nodes.length; i++) nodes[i].next = nodes[i + 1];
  if (pos >= 0) nodes[nodes.length - 1].next = nodes[pos];
  return hasCycle(nodes.length > 0 ? nodes[0] : null);
}
