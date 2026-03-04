-- 1. Rename teacher_rewards to teacher_reward_history
ALTER TABLE teacher_rewards RENAME TO teacher_reward_history;

-- 2. Create teacher_reward_balances table
CREATE TABLE teacher_reward_balances (
    teacher_id TEXT PRIMARY KEY NOT NULL,
    total_points INTEGER NOT NULL DEFAULT 0,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (teacher_id) REFERENCES staff(id)
);

-- 3. Initialize balances from current staff table
INSERT INTO teacher_reward_balances (teacher_id, total_points)
SELECT id, reward_points_balance FROM staff;
