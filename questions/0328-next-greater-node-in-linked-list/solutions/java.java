class Solution {
    public int[] nextLargerNodes(ListNode head) {
        List<Integer> list = new ArrayList<>();
        for (ListNode node = head; node != null; node = node.next) list.add(node.val);
        int n = list.size();
        int[] vals = new int[n];
        for (int i = 0; i < n; i++) vals[i] = list.get(i);
        int[] answer = new int[n];
        int[] waiting = new int[n];
        int top = -1;
        for (int i = 0; i < n; i++) {
            while (top >= 0 && vals[waiting[top]] < vals[i]) {
                answer[waiting[top--]] = vals[i];
            }
            waiting[++top] = i;
        }
        return answer;
    }
}
