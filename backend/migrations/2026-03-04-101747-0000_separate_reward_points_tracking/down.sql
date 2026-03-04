-- 1. Re-add reward_points_balance to staff
CREATE TABLE staff_old (
    id TEXT PRIMARY KEY NOT NULL,
    employee_id TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    nic TEXT NOT NULL UNIQUE,
    dob DATE NOT NULL,
    gender TEXT NOT NULL,
    address TEXT NOT NULL,
    phone TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    photo_url TEXT,
    employment_status TEXT NOT NULL DEFAULT 'Active',
    staff_type TEXT NOT NULL DEFAULT 'Teaching',
    profile_id TEXT,
    reward_points_balance INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (profile_id) REFERENCES profiles(id)
);

INSERT INTO staff_old (id, employee_id, name, nic, dob, gender, address, phone, email, photo_url, employment_status, staff_type, profile_id, reward_points_balance, created_at, updated_at)
SELECT s.id, s.employee_id, s.name, s.nic, s.dob, s.gender, s.address, s.phone, s.email, s.photo_url, s.employment_status, s.staff_type, s.profile_id, COALESCE(b.total_points, 0), s.created_at, s.updated_at 
FROM staff s
LEFT JOIN teacher_reward_balances b ON s.id = b.teacher_id;

DROP TABLE staff;
ALTER TABLE staff_old RENAME TO staff;

-- 2. Drop balances table
DROP TABLE teacher_reward_balances;

-- 3. Rename history back to teacher_rewards
ALTER TABLE teacher_reward_history RENAME TO teacher_rewards;
