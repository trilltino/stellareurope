use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Event {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub event_type: String,
    pub date: DateTime<Utc>,
    pub location: String,
    pub max_participants: Option<i32>,
    pub registration_required: bool,
    pub contact_email: String,
    pub external_link: Option<String>,
    pub organizer_id: i32,
    pub created_at: Option<DateTime<Utc>>,
    // KPI Planning fields
    pub strategic_focus_areas: Option<Vec<String>>,
    pub monthly_active_ambassadors: Option<i32>,
    pub monthly_active_accounts: Option<i32>,
    pub scf_referrals: Option<i32>,
    pub content_produced: Option<i32>,
    pub expected_attendance: Option<i32>,
    pub social_growth_target: Option<i32>,
    pub target_audience: String,
    pub quarterly_goals: String,
    pub strategic_purpose: String,
    pub success_metrics: Option<String>,
}

impl Event {
    pub fn new(
        title: String,
        description: String,
        event_type: String,
        date: DateTime<Utc>,
        location: String,
        max_participants: Option<i32>,
        registration_required: bool,
        contact_email: String,
        external_link: Option<String>,
        organizer_id: i32,
        strategic_focus_areas: Option<Vec<String>>,
        monthly_active_ambassadors: Option<i32>,
        monthly_active_accounts: Option<i32>,
        scf_referrals: Option<i32>,
        content_produced: Option<i32>,
        expected_attendance: Option<i32>,
        social_growth_target: Option<i32>,
        target_audience: String,
        quarterly_goals: String,
        strategic_purpose: String,
        success_metrics: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: 0,
            title,
            description,
            event_type,
            date,
            location,
            max_participants,
            registration_required,
            contact_email,
            external_link,
            organizer_id,
            created_at: Some(now),
            strategic_focus_areas,
            monthly_active_ambassadors,
            monthly_active_accounts,
            scf_referrals,
            content_produced,
            expected_attendance,
            social_growth_target,
            target_audience,
            quarterly_goals,
            strategic_purpose,
            success_metrics,
        }
    }
}