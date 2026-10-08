function sortedListToBst(head: ListNode | null): TreeNode | null {
    let n = 0;
    for (let node = head; node !== null; node = node.next) n++;

    let cur = head;
    function build(lo: number, hi: number): TreeNode | null {
        if (lo > hi) return null;
        const mid = (lo + hi + 1) >> 1;
        const left = build(lo, mid - 1);
        const root = new TreeNode(cur!.val, left, null);
        cur = cur!.next;
        root.right = build(mid + 1, hi);
        return root;
    }
    return build(0, n - 1);
}
