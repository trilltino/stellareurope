-- Add KPI planning fields to events table
ALTER TABLE events ADD COLUMN strategic_focus_areas TEXT[] DEFAULT '{}';
ALTER TABLE events ADD COLUMN monthly_active_ambassadors INTEGER;
ALTER TABLE events ADD COLUMN monthly_active_accounts INTEGER;
ALTER TABLE events ADD COLUMN scf_referrals INTEGER;
ALTER TABLE events ADD COLUMN content_produced INTEGER;
ALTER TABLE events ADD COLUMN expected_attendance INTEGER;
ALTER TABLE events ADD COLUMN social_growth_target INTEGER;
ALTER TABLE events ADD COLUMN target_audience TEXT NOT NULL DEFAULT '';
ALTER TABLE events ADD COLUMN quarterly_goals TEXT NOT NULL DEFAULT '';
ALTER TABLE events ADD COLUMN strategic_purpose TEXT NOT NULL DEFAULT '';
ALTER TABLE events ADD COLUMN success_metrics TEXT;

-- Add indexes for better query performance
CREATE INDEX idx_events_strategic_focus_areas ON events USING GIN(strategic_focus_areas);
CREATE INDEX idx_events_target_audience ON events(target_audience);