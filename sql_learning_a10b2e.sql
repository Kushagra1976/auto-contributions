-- Learning Objective: This tutorial will teach you how to use SQL's powerful string manipulation and pattern matching functions
-- to uncover hidden clues and identify potential patterns within a fictional crime scene dataset.
-- We'll focus on extracting meaningful information from text-based evidence to aid in investigations.

-- Imagine we have a database table called 'crime_scenes' with the following columns:
-- scene_id: A unique identifier for each crime scene.
-- location: The address or general area of the crime scene.
-- description: A textual description of the crime scene, including details about evidence found, witness statements, etc.
-- time_of_incident: The approximate time the incident occurred.

-- For this tutorial, we'll assume you have a database with a 'crime_scenes' table.
-- If you don't, you can create a simple one for practice:
/*
CREATE TABLE crime_scenes (
    scene_id INT PRIMARY KEY,
    location VARCHAR(255),
    description TEXT,
    time_of_incident DATETIME
);

INSERT INTO crime_scenes (scene_id, location, description, time_of_incident) VALUES
(1, '123 Main St', 'Victim found with a single stab wound. A red scarf was discovered near the body.', '2023-10-26 22:30:00'),
(2, '456 Oak Ave', 'Burglary reported. Entry through a forced window. A muddy footprint was noted.', '2023-10-27 01:15:00'),
(3, '789 Pine Ln', 'Assault case. Witness reported seeing a suspect fleeing the scene. The suspect wore a dark hooded jacket.', '2023-10-26 23:00:00'),
(4, '123 Main St', 'Another incident at the same location. A broken vase and scattered papers were found.', '2023-10-27 09:00:00'),
(5, '321 Elm Blvd', 'Theft of a valuable painting. Security camera footage shows a shadowy figure.', '2023-10-27 03:00:00'),
(6, '456 Oak Ave', 'Suspicious activity reported. Neighbors saw a person loitering near a dumpster. Possible evidence might be discarded.', '2023-10-27 00:30:00');
*/

-- Our primary focus will be on the 'description' column, which contains rich textual data.
-- We'll learn how to search for specific keywords and identify patterns using SQL's string functions.

-- Let's start by finding all crime scenes where the word "victim" is mentioned in the description.
-- The LIKE operator is used for pattern matching in strings.
-- The '%' wildcard character matches any sequence of zero or more characters.
SELECT
    scene_id,
    location,
    description
FROM
    crime_scenes
WHERE
    description LIKE '%victim%'; -- This query finds all rows where 'description' contains the substring 'victim'.
                                 -- The '%' before and after 'victim' means it can appear anywhere in the text.

-- Now, let's try to be more specific. What if we're looking for a specific piece of evidence, like a "red scarf"?
SELECT
    scene_id,
    location,
    description
FROM
    crime_scenes
WHERE
    description LIKE '%red scarf%'; -- This narrows down our search to scenes mentioning a "red scarf".

-- We can also combine conditions using the AND operator.
-- Let's find scenes where the location is "123 Main St" AND the description mentions "broken vase".
SELECT
    scene_id,
    location,
    description
FROM
    crime_scenes
WHERE
    location = '123 Main St' AND description LIKE '%broken vase%'; -- Combines two specific criteria.

-- Sometimes, details might be phrased slightly differently. For example, "footprint" or "footprints".
-- We can use the '_' wildcard, which matches any single character, to account for variations.
-- However, for common variations like plurals, it's often easier to use LIKE with multiple OR conditions or more advanced regex if your SQL dialect supports it.
-- For simplicity here, let's assume we're looking for "footprint".
SELECT
    scene_id,
    location,
    description
FROM
    crime_scenes
WHERE
    description LIKE '%footprint%';

-- Let's explore descriptions that mention potential suspects and their attire.
-- We can look for phrases like "hooded jacket" or "dark jacket".
SELECT
    scene_id,
    location,
    description
FROM
    crime_scenes
WHERE
    description LIKE '%hooded jacket%' OR description LIKE '%dark jacket%'; -- Using OR to catch multiple possibilities.

-- We can also use LIKE to find scenes where a specific time range might be relevant.
-- This is illustrative; for precise time filtering, date/time functions are better.
SELECT
    scene_id,
    location,
    description,
    time_of_incident
FROM
    crime_scenes
WHERE
    time_of_incident BETWEEN '2023-10-26 22:00:00' AND '2023-10-27 02:00:00'; -- Finds incidents within a specific timeframe.

-- One powerful technique is to extract specific pieces of information using string functions like SUBSTRING and INSTR (or equivalent).
-- For example, let's assume we have a simplified format where evidence is listed after "evidence: ".
-- Note: The exact syntax for string functions can vary slightly between SQL databases (e.g., MySQL, PostgreSQL, SQL Server, SQLite).
-- The following example uses common SQL concepts.

-- Let's try to extract text that appears AFTER a specific marker, like "found ".
-- We'll use LENGTH to get the length of the string up to "found ", and then SUBSTRING to get the rest.
-- This is a more advanced technique, often requiring careful handling of edge cases.

-- Example: Extracting the part of the description that comes after "discovered near the body."
-- This assumes the phrase appears only once and at the end of the relevant text.
SELECT
    scene_id,
    location,
    description,
    SUBSTRING(description, LENGTH(description) - LENGTH('discovered near the body.') + 1) AS evidence_phrase
FROM
    crime_scenes
WHERE
    description LIKE '%discovered near the body.'; -- First, filter for relevant descriptions.
    -- The SUBSTRING logic here is a bit simplified. A more robust solution might involve INSTR to find the position.
    -- For instance, to get text after "discovered near the body.":
    -- SUBSTRING(description, INSTR(description, 'discovered near the body.') + LENGTH('discovered near the body.') + 1)
    -- This would find the starting position, add the length of the marker, and extract from there.

-- Let's refine that to extract the *specific item* after "discovered near the body.".
-- We'll assume the item is a single word following the phrase.
SELECT
    scene_id,
    location,
    description,
    -- This tries to extract the word immediately following "discovered near the body. "
    -- It's a bit complex and might need adjustment based on actual data.
    TRIM(SUBSTRING(
        description,
        INSTR(description, 'discovered near the body.') + LENGTH('discovered near the body.') + 1,
        -- We're trying to guess the length of the next word. This is fragile!
        -- A better approach would be to find the next space or punctuation after the phrase.
        -- For this example, let's just try to get a few characters, assuming a short word.
        15 -- Arbitrary length, could be refined.
    )) AS potential_evidence_item
FROM
    crime_scenes
WHERE
    description LIKE '%discovered near the body.%';

-- Example Usage:
-- To find all scenes with mentions of "victim":
-- SELECT * FROM crime_scenes WHERE description LIKE '%victim%';

-- To find all scenes at "456 Oak Ave" that might involve discarded items:
-- SELECT * FROM crime_scenes WHERE location = '456 Oak Ave' AND description LIKE '%discarded%';

-- This tutorial introduced basic pattern matching with LIKE and a glimpse into string extraction.
-- These SQL tools are invaluable for sifting through large datasets and uncovering critical details in any investigation.
-- Experiment with different keywords and phrases relevant to your fictional crime scene data!