-- Questions are not versioned: they are edited in place. An attempt records
-- the dojo release it ran on, which pins the exact copy of the question bank.
ALTER TABLE attempts DROP COLUMN question_version;
ALTER TABLE attempts ADD COLUMN dojo_version TEXT NOT NULL DEFAULT '';
