class Solution {
    public int ladderLength(String beginWord, String endWord, String[] wordList) {
        Set<String> words = new HashSet<>(Arrays.asList(wordList));
        if (!words.contains(endWord)) return 0;
        words.remove(beginWord);
        Deque<String> queue = new ArrayDeque<>();
        Deque<Integer> stepsQueue = new ArrayDeque<>();
        queue.add(beginWord);
        stepsQueue.add(1);
        while (!queue.isEmpty()) {
            char[] word = queue.poll().toCharArray();
            int steps = stepsQueue.poll();
            for (int i = 0; i < word.length; i++) {
                char original = word[i];
                for (char ch = 'a'; ch <= 'z'; ch++) {
                    word[i] = ch;
                    String candidate = new String(word);
                    if (words.contains(candidate)) {
                        if (candidate.equals(endWord)) return steps + 1;
                        words.remove(candidate); // visited: never enqueue twice
                        queue.add(candidate);
                        stepsQueue.add(steps + 1);
                    }
                }
                word[i] = original;
            }
        }
        return 0;
    }
}
