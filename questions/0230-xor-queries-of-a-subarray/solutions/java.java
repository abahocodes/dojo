class Solution {
    public int[] xorQueries(int[] arr, int[][] queries) {
        int[] prefix = new int[arr.length + 1];
        for (int i = 0; i < arr.length; i++) prefix[i + 1] = prefix[i] ^ arr[i];
        int[] answers = new int[queries.length];
        for (int i = 0; i < queries.length; i++) {
            answers[i] = prefix[queries[i][1] + 1] ^ prefix[queries[i][0]];
        }
        return answers;
    }
}
