class Solution {
    public int[] cycleLengthQueries(int n, int[][] queries) {
        int[] answer = new int[queries.length];
        for (int i = 0; i < queries.length; i++) {
            int a = queries[i][0], b = queries[i][1];
            int steps = 0;
            while (a != b) {
                if (a > b) a >>= 1;
                else b >>= 1;
                steps++;
            }
            answer[i] = steps + 1;
        }
        return answer;
    }
}
