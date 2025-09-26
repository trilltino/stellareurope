use crate::database::models::Event;
use crate::database::connection::DbPool;
use sqlx::{Error as SqlxError};
use chrono::{DateTime, Utc};

pub struct EventRepository;

impl EventRepository {
    pub async fn create_event(
        pool: &DbPool,
        title: &str,
        description: &str,
        event_type: &str,
        date: DateTime<Utc>,
        location: &str,
        max_participants: Option<i32>,
        registration_required: bool,
        contact_email: &str,
        external_link: Option<&str>,
        organizer_id: i32,
        strategic_focus_areas: Option<&Vec<String>>,
        monthly_active_ambassadors: Option<i32>,
        monthly_active_accounts: Option<i32>,
        scf_referrals: Option<i32>,
        content_produced: Option<i32>,
        expected_attendance: Option<i32>,
        social_growth_target: Option<i32>,
        target_audience: &str,
        quarterly_goals: &str,
        strategic_purpose: &str,
        success_metrics: Option<&str>,
    ) -> Result<Event, SqlxError> {
        let row = sqlx::query!(
            r#"
            INSERT INTO events (title, description, event_type, date, location, max_participants, registration_required, contact_email, external_link, organizer_id,
                              strategic_focus_areas, monthly_active_ambassadors, monthly_active_accounts, scf_referrals, content_produced, expected_attendance,
                              social_growth_target, target_audience, quarterly_goals, strategic_purpose, success_metrics, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, NOW())
            RETURNING id, title, description, event_type, date, location, max_participants, registration_required, contact_email, external_link, organizer_id,
                      strategic_focus_areas, monthly_active_ambassadors, monthly_active_accounts, scf_referrals, content_produced, expected_attendance,
                      social_growth_target, target_audience, quarterly_goals, strategic_purpose, success_metrics, created_at
            "#,
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
            strategic_focus_areas.map(|v| v.as_slice()),
            monthly_active_ambassadors,
            monthly_active_accounts,
            scf_referrals,
            content_produced,
            expected_attendance,
            social_growth_target,
            target_audience,
            quarterly_goals,
            strategic_purpose,
            success_metrics
        )
        .fetch_one(pool)
        .await?;

        Ok(Event {
            id: row.id,
            title: row.title,
            description: row.description,
            event_type: row.event_type,
            date: row.date,
            location: row.location,
            max_participants: row.max_participants,
            registration_required: row.registration_required,
            contact_email: row.contact_email,
            external_link: row.external_link,
            organizer_id: row.organizer_id,
            created_at: row.created_at,
            strategic_focus_areas: row.strategic_focus_areas,
            monthly_active_ambassadors: row.monthly_active_ambassadors,
            monthly_active_accounts: row.monthly_active_accounts,
            scf_referrals: row.scf_referrals,
            content_produced: row.content_produced,
            expected_attendance: row.expected_attendance,
            social_growth_target: row.social_growth_target,
            target_audience: row.target_audience,
            quarterly_goals: row.quarterly_goals,
            strategic_purpose: row.strategic_purpose,
            success_metrics: row.success_metrics,
        })
    }

    pub async fn list_events(
        pool: &DbPool,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Event>, SqlxError> {
        let limit = limit.unwrap_or(50);
        let offset = offset.unwrap_or(0);

        let rows = sqlx::query!(
            r#"
            SELECT id, title, description, event_type, date, location, max_participants,
                   registration_required, contact_email, external_link, organizer_id, created_at,
                   strategic_focus_areas, monthly_active_ambassadors, monthly_active_accounts, scf_referrals,
                   content_produced, expected_attendance, social_growth_target, target_audience,
                   quarterly_goals, strategic_purpose, success_metrics
            FROM events
            ORDER BY date ASC
            LIMIT $1 OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;

        let events = rows.into_iter().map(|row| Event {
            id: row.id,
            title: row.title,
            description: row.description,
            event_type: row.event_type,
            date: row.date,
            location: row.location,
            max_participants: row.max_participants,
            registration_required: row.registration_required,
            contact_email: row.contact_email,
            external_link: row.external_link,
            organizer_id: row.organizer_id,
            created_at: row.created_at,
            strategic_focus_areas: row.strategic_focus_areas,
            monthly_active_ambassadors: row.monthly_active_ambassadors,
            monthly_active_accounts: row.monthly_active_accounts,
            scf_referrals: row.scf_referrals,
            content_produced: row.content_produced,
            expected_attendance: row.expected_attendance,
            social_growth_target: row.social_growth_target,
            target_audience: row.target_audience,
            quarterly_goals: row.quarterly_goals,
            strategic_purpose: row.strategic_purpose,
            success_metrics: row.success_metrics,
        }).collect();

        Ok(events)
    }

    pub async fn find_by_id(
        pool: &DbPool,
        event_id: i32,
    ) -> Result<Option<Event>, SqlxError> {
        let row = sqlx::query!(
            r#"
            SELECT id, title, description, event_type, date, location, max_participants,
                   registration_required, contact_email, external_link, organizer_id, created_at,
                   strategic_focus_areas, monthly_active_ambassadors, monthly_active_accounts, scf_referrals,
                   content_produced, expected_attendance, social_growth_target, target_audience,
                   quarterly_goals, strategic_purpose, success_metrics
            FROM events WHERE id = $1
            "#,
            event_id
        )
        .fetch_optional(pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(Event {
                id: row.id,
                title: row.title,
                description: row.description,
                event_type: row.event_type,
                date: row.date,
                location: row.location,
                max_participants: row.max_participants,
                registration_required: row.registration_required,
                contact_email: row.contact_email,
                external_link: row.external_link,
                organizer_id: row.organizer_id,
                created_at: row.created_at,
                strategic_focus_areas: row.strategic_focus_areas,
                monthly_active_ambassadors: row.monthly_active_ambassadors,
                monthly_active_accounts: row.monthly_active_accounts,
                scf_referrals: row.scf_referrals,
                content_produced: row.content_produced,
                expected_attendance: row.expected_attendance,
                social_growth_target: row.social_growth_target,
                target_audience: row.target_audience,
                quarterly_goals: row.quarterly_goals,
                strategic_purpose: row.strategic_purpose,
                success_metrics: row.success_metrics,
            }))
        } else {
            Ok(None)
        }
    }
}