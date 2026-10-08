function minMeetingRooms(intervals) {
  const starts = intervals.map(([s]) => s).sort((a, b) => a - b);
  const ends = intervals.map(([, e]) => e).sort((a, b) => a - b);
  let rooms = 0;
  let j = 0; // ends[j] is the earliest end time that hasn't freed a room yet
  for (const s of starts) {
    if (s >= ends[j]) {
      j++;
    } else {
      rooms++;
    }
  }
  return rooms;
}
