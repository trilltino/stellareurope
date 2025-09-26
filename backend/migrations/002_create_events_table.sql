-- Create events table
CREATE TABLE events (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    event_type VARCHAR(50) NOT NULL CHECK (event_type IN ('Workshop', 'Meetup', 'Conference', 'Hackathon', 'Community')),
    date TIMESTAMP WITH TIME ZONE NOT NULL,
    location VARCHAR(255) NOT NULL,
    max_participants INTEGER,
    registration_required BOOLEAN NOT NULL DEFAULT false,
    contact_email VARCHAR(255) NOT NULL,
    external_link VARCHAR(500),
    organizer_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_events_date ON events(date);
CREATE INDEX idx_events_event_type ON events(event_type);
CREATE INDEX idx_events_organizer_id ON events(organizer_id);
CREATE INDEX idx_events_created_at ON events(created_at);