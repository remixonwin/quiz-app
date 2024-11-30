-- First, create an admin user if not exists
INSERT INTO users (username, email, password_hash)
VALUES (
    'admin',
    'admin@example.com',
    '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewFyGQQwHWKTZnuK'  -- password: admin123
) ON CONFLICT (email) DO NOTHING;

-- Create a quiz for DMV questions
INSERT INTO quizzes (title, description, creator_id)
VALUES (
    'Minnesota DMV Practice Test',
    'Practice questions based on the Minnesota Driver''s Manual. Test your knowledge of road rules, traffic signs, and safe driving practices.',
    (SELECT id FROM users WHERE email = 'admin@example.com')
) ON CONFLICT DO NOTHING;

-- Questions and answers based on MN DMV Manual
INSERT INTO questions (quiz_id, text, question_type, order_num)
VALUES 
    -- Road Signs
    ((SELECT id FROM quizzes WHERE title = 'Minnesota DMV Practice Test'), 
    'What does an octagonal sign always mean?', 'multiple_choice', 1),
    ((SELECT id FROM quizzes WHERE title = 'Minnesota DMV Practice Test'), 
    'What does a yellow diamond-shaped sign indicate?', 'multiple_choice', 2),
    ((SELECT id FROM quizzes WHERE title = 'Minnesota DMV Practice Test'), 
    'What color are construction zone signs?', 'multiple_choice', 3),

    -- Speed and Safety
    ((SELECT id FROM quizzes WHERE title = 'Minnesota DMV Practice Test'), 
    'What is the speed limit in urban residential areas unless otherwise posted?', 'multiple_choice', 4),
    ((SELECT id FROM quizzes WHERE title = 'Minnesota DMV Practice Test'), 
    'When driving in fog, you should use:', 'multiple_choice', 5),
    ((SELECT id FROM quizzes WHERE title = 'Minnesota DMV Practice Test'), 
    'What is the minimum safe following distance in good weather conditions?', 'multiple_choice', 6),

    -- Right of Way
    ((SELECT id FROM quizzes WHERE title = 'Minnesota DMV Practice Test'), 
    'At a four-way stop, who has the right of way?', 'multiple_choice', 7),
    ((SELECT id FROM quizzes WHERE title = 'Minnesota DMV Practice Test'), 
    'When must you yield to a pedestrian?', 'multiple_choice', 8),
    ((SELECT id FROM quizzes WHERE title = 'Minnesota DMV Practice Test'), 
    'What should you do when approaching an emergency vehicle with sirens on?', 'multiple_choice', 9),

    -- Winter Driving
    ((SELECT id FROM quizzes WHERE title = 'Minnesota DMV Practice Test'), 
    'When driving on snow or ice, what is the best way to stop?', 'multiple_choice', 10);

-- Answers for Road Signs
INSERT INTO answers (question_id, text, is_correct, order_num)
VALUES
    ((SELECT id FROM questions WHERE text = 'What does an octagonal sign always mean?'),
    'Stop', true, 1),
    ((SELECT id FROM questions WHERE text = 'What does an octagonal sign always mean?'),
    'Yield', false, 2),
    ((SELECT id FROM questions WHERE text = 'What does an octagonal sign always mean?'),
    'Slow down', false, 3),
    ((SELECT id FROM questions WHERE text = 'What does an octagonal sign always mean?'),
    'Merge', false, 4),

    ((SELECT id FROM questions WHERE text = 'What does a yellow diamond-shaped sign indicate?'),
    'Warning', true, 1),
    ((SELECT id FROM questions WHERE text = 'What does a yellow diamond-shaped sign indicate?'),
    'Stop ahead', false, 2),
    ((SELECT id FROM questions WHERE text = 'What does a yellow diamond-shaped sign indicate?'),
    'Speed limit', false, 3),
    ((SELECT id FROM questions WHERE text = 'What does a yellow diamond-shaped sign indicate?'),
    'Construction', false, 4),

    ((SELECT id FROM questions WHERE text = 'What color are construction zone signs?'),
    'Orange', true, 1),
    ((SELECT id FROM questions WHERE text = 'What color are construction zone signs?'),
    'Yellow', false, 2),
    ((SELECT id FROM questions WHERE text = 'What color are construction zone signs?'),
    'Red', false, 3),
    ((SELECT id FROM questions WHERE text = 'What color are construction zone signs?'),
    'Green', false, 4),

    -- Answers for Speed and Safety
    ((SELECT id FROM questions WHERE text = 'What is the speed limit in urban residential areas unless otherwise posted?'),
    '30 mph', true, 1),
    ((SELECT id FROM questions WHERE text = 'What is the speed limit in urban residential areas unless otherwise posted?'),
    '25 mph', false, 2),
    ((SELECT id FROM questions WHERE text = 'What is the speed limit in urban residential areas unless otherwise posted?'),
    '35 mph', false, 3),
    ((SELECT id FROM questions WHERE text = 'What is the speed limit in urban residential areas unless otherwise posted?'),
    '40 mph', false, 4),

    ((SELECT id FROM questions WHERE text = 'When driving in fog, you should use:'),
    'Low beam headlights', true, 1),
    ((SELECT id FROM questions WHERE text = 'When driving in fog, you should use:'),
    'High beam headlights', false, 2),
    ((SELECT id FROM questions WHERE text = 'When driving in fog, you should use:'),
    'Parking lights only', false, 3),
    ((SELECT id FROM questions WHERE text = 'When driving in fog, you should use:'),
    'Hazard lights', false, 4),

    ((SELECT id FROM questions WHERE text = 'What is the minimum safe following distance in good weather conditions?'),
    '3 seconds', true, 1),
    ((SELECT id FROM questions WHERE text = 'What is the minimum safe following distance in good weather conditions?'),
    '1 second', false, 2),
    ((SELECT id FROM questions WHERE text = 'What is the minimum safe following distance in good weather conditions?'),
    '2 seconds', false, 3),
    ((SELECT id FROM questions WHERE text = 'What is the minimum safe following distance in good weather conditions?'),
    '4 seconds', false, 4),

    -- Answers for Right of Way
    ((SELECT id FROM questions WHERE text = 'At a four-way stop, who has the right of way?'),
    'The vehicle that arrived first', true, 1),
    ((SELECT id FROM questions WHERE text = 'At a four-way stop, who has the right of way?'),
    'The vehicle on the right', false, 2),
    ((SELECT id FROM questions WHERE text = 'At a four-way stop, who has the right of way?'),
    'The larger vehicle', false, 3),
    ((SELECT id FROM questions WHERE text = 'At a four-way stop, who has the right of way?'),
    'The vehicle going straight', false, 4),

    ((SELECT id FROM questions WHERE text = 'When must you yield to a pedestrian?'),
    'At all marked or unmarked crosswalks', true, 1),
    ((SELECT id FROM questions WHERE text = 'When must you yield to a pedestrian?'),
    'Only at marked crosswalks', false, 2),
    ((SELECT id FROM questions WHERE text = 'When must you yield to a pedestrian?'),
    'Only when traffic lights are present', false, 3),
    ((SELECT id FROM questions WHERE text = 'When must you yield to a pedestrian?'),
    'Only during daylight hours', false, 4),

    ((SELECT id FROM questions WHERE text = 'What should you do when approaching an emergency vehicle with sirens on?'),
    'Pull over to the right and stop', true, 1),
    ((SELECT id FROM questions WHERE text = 'What should you do when approaching an emergency vehicle with sirens on?'),
    'Speed up to get out of the way', false, 2),
    ((SELECT id FROM questions WHERE text = 'What should you do when approaching an emergency vehicle with sirens on?'),
    'Continue at the same speed', false, 3),
    ((SELECT id FROM questions WHERE text = 'What should you do when approaching an emergency vehicle with sirens on?'),
    'Slow down but keep moving', false, 4),

    -- Answers for Winter Driving
    ((SELECT id FROM questions WHERE text = 'When driving on snow or ice, what is the best way to stop?'),
    'Pump the brakes gently if you don''t have anti-lock brakes', true, 1),
    ((SELECT id FROM questions WHERE text = 'When driving on snow or ice, what is the best way to stop?'),
    'Slam on the brakes', false, 2),
    ((SELECT id FROM questions WHERE text = 'When driving on snow or ice, what is the best way to stop?'),
    'Pull the emergency brake', false, 3),
    ((SELECT id FROM questions WHERE text = 'When driving on snow or ice, what is the best way to stop?'),
    'Turn the wheel in the opposite direction', false, 4);
