class Solution {
    public int numRescueBoats(int[] people, int limit) {
        int[] p = people.clone();
        Arrays.sort(p);
        int i = 0, j = p.length - 1, boats = 0;
        while (i <= j) {
            if (p[i] + p[j] <= limit) i++;
            j--;
            boats++;
        }
        return boats;
    }
}
