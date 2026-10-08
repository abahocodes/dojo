class Solution {
    public String getHint(String secret, String guess) {
        int bulls = 0, cows = 0;
        int[] bal = new int[10];
        for (int i = 0; i < secret.length(); i++) {
            int a = secret.charAt(i) - '0';
            int b = guess.charAt(i) - '0';
            if (a == b) { bulls++; continue; }
            if (bal[a] < 0) cows++;
            if (bal[b] > 0) cows++;
            bal[a]++;
            bal[b]--;
        }
        return bulls + "A" + cows + "B";
    }
}
