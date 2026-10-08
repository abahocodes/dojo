class Solution {
    public int furthestBuilding(int[] heights, int bricks, int ladders) {
        PriorityQueue<Integer> ladderClimbs = new PriorityQueue<>();
        for (int i = 0; i + 1 < heights.length; i++) {
            int climb = heights[i + 1] - heights[i];
            if (climb <= 0) continue;
            ladderClimbs.add(climb);
            if (ladderClimbs.size() > ladders) {
                bricks -= ladderClimbs.poll();
                if (bricks < 0) return i;
            }
        }
        return heights.length - 1;
    }
}
