// 1003. Time Based Key-Value Store (medium)
// /show to reread the problem · /test runs visible tests · /submit when done

class TimeMap {
    public TimeMap() {

    }

    public void set(String key, String value, int timestamp) {

    }

    public String get(String key, int timestamp) {
        return "";
    }
}

class Solution {
    // dojo calls this to run the tests; you don't need to change it.
    public String[] timeMap(String[] ops, String[] keys, String[] values, int[] timestamps) {
        TimeMap store = new TimeMap();
        List<String> out = new ArrayList<>();
        for (int i = 0; i < ops.length; i++) {
            if (ops[i].equals("set")) store.set(keys[i], values[i], timestamps[i]);
            else out.add(store.get(keys[i], timestamps[i]));
        }
        return out.toArray(new String[0]);
    }
}
