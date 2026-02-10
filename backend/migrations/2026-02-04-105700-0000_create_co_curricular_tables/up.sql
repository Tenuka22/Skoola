-- 9.1 Sports Management
CREATE TABLE sports (
    id TEXT PRIMARY KEY NOT NULL,
    sport_name TEXT NOT NULL,
    description TEXT,
    category TEXT NOT NULL, -- e.g., Indoor, Outdoor, Team, Individual
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE sport_teams (
    id TEXT PRIMARY KEY NOT NULL,
    sport_id TEXT NOT NULL REFERENCES sports(id),
    team_name TEXT NOT NULL,
    grade_level TEXT NOT NULL,
    coach_id TEXT NOT NULL REFERENCES staff(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE sport_team_members (
    team_id TEXT NOT NULL REFERENCES sport_teams(id),
    student_id TEXT NOT NULL REFERENCES students(id),
    position TEXT,
    joined_date DATE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (team_id, student_id)
);

CREATE TABLE sport_events (
    id TEXT PRIMARY KEY NOT NULL,
    sport_id TEXT NOT NULL REFERENCES sports(id),
    event_name TEXT NOT NULL,
    event_date TIMESTAMP NOT NULL,
    venue TEXT NOT NULL,
    organizer TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE sport_event_participants (
    event_id TEXT NOT NULL REFERENCES sport_events(id),
    student_id TEXT NOT NULL REFERENCES students(id),
    team_id TEXT REFERENCES sport_teams(id),
    position TEXT,
    points INTEGER,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (event_id, student_id)
);

-- 9.2 Clubs & Societies Management
CREATE TABLE clubs (
    id TEXT PRIMARY KEY NOT NULL,
    club_name TEXT NOT NULL,
    description TEXT,
    teacher_in_charge_id TEXT NOT NULL REFERENCES staff(id),
    meeting_schedule TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE club_members (
    club_id TEXT NOT NULL REFERENCES clubs(id),
    student_id TEXT NOT NULL REFERENCES students(id),
    role TEXT NOT NULL, -- e.g., President, Secretary, Member
    joined_date DATE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (club_id, student_id)
);

CREATE TABLE club_activities (
    id TEXT PRIMARY KEY NOT NULL,
    club_id TEXT NOT NULL REFERENCES clubs(id),
    activity_name TEXT NOT NULL,
    activity_date TIMESTAMP NOT NULL,
    description TEXT,
    participants_count INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 9.3 Competitions & Achievements
CREATE TABLE competitions (
    id TEXT PRIMARY KEY NOT NULL,
    competition_name TEXT NOT NULL,
    competition_type TEXT NOT NULL,
    date TIMESTAMP NOT NULL,
    organizer TEXT NOT NULL,
    level TEXT NOT NULL, -- School, District, Provincial, National, International
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE competition_participants (
    competition_id TEXT NOT NULL REFERENCES competitions(id),
    student_id TEXT NOT NULL REFERENCES students(id),
    position TEXT, -- e.g., 1st, 2nd, 3rd
    award TEXT, -- e.g., Gold Medal, Certificate
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (competition_id, student_id)
);

CREATE TABLE student_achievements (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL REFERENCES students(id),
    achievement_type TEXT NOT NULL, -- Sports, Academic, Extra-curricular
    description TEXT NOT NULL,
    date DATE NOT NULL,
    certificate_url TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 9.4 Cultural Activities
CREATE TABLE cultural_events (
    id TEXT PRIMARY KEY NOT NULL,
    event_name TEXT NOT NULL,
    event_date TIMESTAMP NOT NULL,
    venue TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE cultural_event_participants (
    event_id TEXT NOT NULL REFERENCES cultural_events(id),
    student_id TEXT NOT NULL REFERENCES students(id),
    performance_type TEXT NOT NULL, -- e.g., Dance, Music, Drama
    role TEXT, -- e.g., Main Character, Backstage
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (event_id, student_id)
);
