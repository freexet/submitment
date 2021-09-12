SELECT s.id, s.reviewer_id, s.question, s.answer, s.score, s.created_at, s.updated_at, u.id as user_id, u.username, u.created_at as user_created_at
FROM submissions AS s
INNER JOIN users as u ON s.user_id = u.id
WHERE s.deleted_at IS NULL
ORDER BY s.created_at DESC