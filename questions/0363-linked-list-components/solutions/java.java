class Solution {
    public int numComponents(ListNode head, int[] nums) {
        Set<Integer> wanted = new HashSet<>();
        for (int x : nums) wanted.add(x);
        int count = 0;
        for (ListNode cur = head; cur != null; cur = cur.next) {
            if (wanted.contains(cur.val) && (cur.next == null || !wanted.contains(cur.next.val))) count++;
        }
        return count;
    }
}
