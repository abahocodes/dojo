class Solution {
    public String[] removeSubfolders(String[] folder) {
        String[] sorted = folder.clone();
        Arrays.sort(sorted);
        List<String> result = new ArrayList<>();
        for (String path : sorted) {
            if (result.isEmpty() || !path.startsWith(result.get(result.size() - 1) + "/")) {
                result.add(path);
            }
        }
        return result.toArray(new String[0]);
    }
}
