class Solution {
    public boolean hasGroupsSizeX(int[] deck) {
        int[] count = new int[10000];
        for (int v : deck) count[v]++;
        int g = 0;
        for (int c : count) {
            if (c > 0) g = gcd(g, c);
        }
        return g >= 2;
    }

    private int gcd(int a, int b) {
        while (b != 0) {
            int t = a % b;
            a = b;
            b = t;
        }
        return a;
    }
}
