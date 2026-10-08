class Solution {
    public int bestClosingTime(String customers) {
        int delta = 0;
        int bestDelta = 0;
        int bestHour = 0;
        for (int i = 0; i < customers.length(); i++) {
            delta += customers.charAt(i) == 'Y' ? -1 : 1;
            if (delta < bestDelta) {
                bestDelta = delta;
                bestHour = i + 1;
            }
        }
        return bestHour;
    }
}
