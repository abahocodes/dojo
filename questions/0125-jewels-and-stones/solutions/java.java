class Solution {
    public int numJewelsInStones(String jewels, String stones) {
        boolean[] kinds = new boolean[128];
        for (char c : jewels.toCharArray()) kinds[c] = true;
        int count = 0;
        for (char c : stones.toCharArray()) if (kinds[c]) count++;
        return count;
    }
}
