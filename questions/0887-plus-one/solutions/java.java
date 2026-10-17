class Solution {
    public int[] plusOne(int[] digits) {
        // Walk from the last digit: a 9 becomes 0 and carries; anything else
        // just increments and we're done.
        int[] result = digits.clone();
        for (int i = result.length - 1; i >= 0; i--) {
            if (result[i] < 9) {
                result[i]++;
                return result;
            }
            result[i] = 0;
        }
        // Every digit was 9: the answer is 1 followed by zeros.
        int[] longer = new int[result.length + 1];
        longer[0] = 1;
        return longer;
    }
}
