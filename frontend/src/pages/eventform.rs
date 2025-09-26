use yew::prelude::*;
use web_sys::HtmlInputElement;
use shared::dto::{EventRequest, EventType, StrategicFocusArea, KPIEstimates};
use crate::services::api;

#[derive(PartialEq, Clone)]
pub enum EventFormState {
    Form,
    Loading,
    Success(String),
    Error(String),
}

#[function_component(EventFormPage)]
pub fn event_form_page() -> Html {
    let state = use_state(|| EventFormState::Form);
    let title = use_state(|| String::new());
    let description = use_state(|| String::new());
    let event_type = use_state(|| EventType::Meetup);
    let date = use_state(|| String::new());
    let location = use_state(|| String::new());
    let max_participants = use_state(|| String::new());
    let registration_required = use_state(|| false);
    let contact_email = use_state(|| String::new());
    let external_link = use_state(|| String::new());

    // KPI Planning fields
    let strategic_focus_areas = use_state(|| vec![false, false, false, false, false]); // [community, onchain, scf, ecosystem, developer]
    let monthly_active_ambassadors = use_state(|| String::new());
    let monthly_active_accounts = use_state(|| String::new());
    let scf_referrals = use_state(|| String::new());
    let content_produced = use_state(|| String::new());
    let expected_attendance = use_state(|| String::new());
    let social_growth_target = use_state(|| String::new());
    let target_audience = use_state(|| String::new());
    let quarterly_goals = use_state(|| String::new());
    let strategic_purpose = use_state(|| String::new());
    let success_metrics = use_state(|| String::new());

    let on_title_change = {
        let title = title.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            title.set(input.value());
        })
    };

    let on_description_change = {
        let description = description.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            description.set(input.value());
        })
    };

    let on_event_type_change = {
        let event_type = event_type.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let selected_type = match input.value().as_str() {
                "Workshop" => EventType::Workshop,
                "Conference" => EventType::Conference,
                "Hackathon" => EventType::Hackathon,
                "Community" => EventType::Community,
                _ => EventType::Meetup,
            };
            event_type.set(selected_type);
        })
    };

    let on_date_change = {
        let date = date.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            date.set(input.value());
        })
    };

    let on_location_change = {
        let location = location.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            location.set(input.value());
        })
    };

    let on_max_participants_change = {
        let max_participants = max_participants.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            max_participants.set(input.value());
        })
    };

    let on_registration_change = {
        let registration_required = registration_required.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            registration_required.set(input.checked());
        })
    };

    let on_email_change = {
        let contact_email = contact_email.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            contact_email.set(input.value());
        })
    };

    let on_link_change = {
        let external_link = external_link.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            external_link.set(input.value());
        })
    };

    // KPI callbacks
    let on_strategic_focus_change = {
        let strategic_focus_areas = strategic_focus_areas.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let index = input.value().parse::<usize>().unwrap_or(0);
            let mut areas = (*strategic_focus_areas).clone();
            areas[index] = input.checked();
            strategic_focus_areas.set(areas);
        })
    };

    let on_monthly_ambassadors_change = {
        let monthly_active_ambassadors = monthly_active_ambassadors.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            monthly_active_ambassadors.set(input.value());
        })
    };

    let on_monthly_accounts_change = {
        let monthly_active_accounts = monthly_active_accounts.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            monthly_active_accounts.set(input.value());
        })
    };

    let on_scf_referrals_change = {
        let scf_referrals = scf_referrals.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            scf_referrals.set(input.value());
        })
    };

    let on_content_produced_change = {
        let content_produced = content_produced.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            content_produced.set(input.value());
        })
    };

    let on_expected_attendance_change = {
        let expected_attendance = expected_attendance.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            expected_attendance.set(input.value());
        })
    };

    let on_social_growth_change = {
        let social_growth_target = social_growth_target.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            social_growth_target.set(input.value());
        })
    };

    let on_target_audience_change = {
        let target_audience = target_audience.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            target_audience.set(input.value());
        })
    };

    let on_quarterly_goals_change = {
        let quarterly_goals = quarterly_goals.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            quarterly_goals.set(input.value());
        })
    };

    let on_strategic_purpose_change = {
        let strategic_purpose = strategic_purpose.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            strategic_purpose.set(input.value());
        })
    };

    let on_success_metrics_change = {
        let success_metrics = success_metrics.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            success_metrics.set(input.value());
        })
    };

    let on_submit = {
        let state = state.clone();
        let title = title.clone();
        let description = description.clone();
        let event_type = event_type.clone();
        let date = date.clone();
        let location = location.clone();
        let max_participants = max_participants.clone();
        let registration_required = registration_required.clone();
        let contact_email = contact_email.clone();
        let external_link = external_link.clone();
        let strategic_focus_areas = strategic_focus_areas.clone();
        let monthly_active_ambassadors = monthly_active_ambassadors.clone();
        let monthly_active_accounts = monthly_active_accounts.clone();
        let scf_referrals = scf_referrals.clone();
        let content_produced = content_produced.clone();
        let expected_attendance = expected_attendance.clone();
        let social_growth_target = social_growth_target.clone();
        let target_audience = target_audience.clone();
        let quarterly_goals = quarterly_goals.clone();
        let strategic_purpose = strategic_purpose.clone();
        let success_metrics = success_metrics.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let max_participants_num = if max_participants.is_empty() {
                None
            } else {
                max_participants.parse::<u32>().ok()
            };

            // Map strategic focus areas
            let focus_areas: Vec<StrategicFocusArea> = strategic_focus_areas.iter().enumerate()
                .filter_map(|(index, &selected)| {
                    if selected {
                        match index {
                            0 => Some(StrategicFocusArea::CommunityParticipation),
                            1 => Some(StrategicFocusArea::OnChainActivity),
                            2 => Some(StrategicFocusArea::SCFReferrals),
                            3 => Some(StrategicFocusArea::EcosystemCollaboration),
                            4 => Some(StrategicFocusArea::DeveloperGrowth),
                            _ => None,
                        }
                    } else {
                        None
                    }
                })
                .collect();

            let kpi_estimates = KPIEstimates {
                monthly_active_ambassadors: monthly_active_ambassadors.parse().ok(),
                monthly_active_accounts: monthly_active_accounts.parse().ok(),
                scf_referrals: scf_referrals.parse().ok(),
                content_produced: content_produced.parse().ok(),
                expected_attendance: expected_attendance.parse().ok(),
                social_growth_target: social_growth_target.parse().ok(),
            };

            let request = EventRequest {
                title: (*title).clone(),
                description: (*description).clone(),
                event_type: (*event_type).clone(),
                date: (*date).clone(),
                location: (*location).clone(),
                max_participants: max_participants_num,
                registration_required: *registration_required,
                contact_email: (*contact_email).clone(),
                external_link: if external_link.is_empty() { None } else { Some((*external_link).clone()) },
                strategic_focus_areas: focus_areas,
                kpi_estimates,
                target_audience: (*target_audience).clone(),
                quarterly_goals: (*quarterly_goals).clone(),
                strategic_purpose: (*strategic_purpose).clone(),
                success_metrics: if success_metrics.is_empty() { None } else { Some((*success_metrics).clone()) },
            };

            state.set(EventFormState::Loading);

            let state_clone = state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match api::create_event(request).await {
                    Ok(message) => {
                        state_clone.set(EventFormState::Success(message));
                    }
                    Err(e) => {
                        state_clone.set(EventFormState::Error(format!("Event creation failed: {}", e)));
                    }
                }
            });
        })
    };

    html! {
        <div class="event-form-container">
            <div class="event-form-card">
                <h1 class="form-title">{"Create New Event"}</h1>
                <p class="form-subtitle">{"Organize your next Stellar community event"}</p>

                {match &*state {
                    EventFormState::Form => html! {
                        <form class="event-form" onsubmit={on_submit}>
                            <div class="form-section">
                                <h2 class="section-title">{"Event Details"}</h2>

                                <div class="form-group">
                                    <label for="title">{"Event Title *"}</label>
                                    <input
                                        type="text"
                                        id="title"
                                        value={(*title).clone()}
                                        onchange={on_title_change}
                                        placeholder="Enter event title"
                                        required=true
                                    />
                                </div>

                                <div class="form-group">
                                    <label for="description">{"Description *"}</label>
                                    <textarea
                                        id="description"
                                        value={(*description).clone()}
                                        onchange={on_description_change}
                                        placeholder="Describe your event, agenda, speakers, etc."
                                        rows="4"
                                        required=true
                                    ></textarea>
                                </div>

                                <div class="form-row">
                                    <div class="form-group">
                                        <label for="event-type">{"Event Type *"}</label>
                                        <select id="event-type" onchange={on_event_type_change}>
                                            <option value="Meetup" selected={*event_type == EventType::Meetup}>{"Meetup"}</option>
                                            <option value="Workshop" selected={*event_type == EventType::Workshop}>{"Workshop"}</option>
                                            <option value="Conference" selected={*event_type == EventType::Conference}>{"Conference"}</option>
                                            <option value="Hackathon" selected={*event_type == EventType::Hackathon}>{"Hackathon"}</option>
                                            <option value="Community" selected={*event_type == EventType::Community}>{"Community Event"}</option>
                                        </select>
                                    </div>

                                    <div class="form-group">
                                        <label for="date">{"Date & Time *"}</label>
                                        <input
                                            type="datetime-local"
                                            id="date"
                                            value={(*date).clone()}
                                            onchange={on_date_change}
                                            required=true
                                        />
                                    </div>
                                </div>

                                <div class="form-group">
                                    <label for="location">{"Location *"}</label>
                                    <input
                                        type="text"
                                        id="location"
                                        value={(*location).clone()}
                                        onchange={on_location_change}
                                        placeholder="Venue name, address, or 'Online'"
                                        required=true
                                    />
                                </div>
                            </div>

                            <div class="form-section">
                                <h2 class="section-title">{"Registration & Contact"}</h2>

                                <div class="form-row">
                                    <div class="form-group">
                                        <label for="max-participants">{"Max Participants"}</label>
                                        <input
                                            type="number"
                                            id="max-participants"
                                            value={(*max_participants).clone()}
                                            onchange={on_max_participants_change}
                                            placeholder="Leave empty for unlimited"
                                            min="1"
                                        />
                                    </div>

                                    <div class="form-group checkbox-group">
                                        <label class="checkbox-label">
                                            <input
                                                type="checkbox"
                                                checked={*registration_required}
                                                onchange={on_registration_change}
                                            />
                                            <span class="checkmark"></span>
                                            {"Registration Required"}
                                        </label>
                                    </div>
                                </div>

                                <div class="form-group">
                                    <label for="contact-email">{"Contact Email *"}</label>
                                    <input
                                        type="email"
                                        id="contact-email"
                                        value={(*contact_email).clone()}
                                        onchange={on_email_change}
                                        placeholder="organizer@example.com"
                                        required=true
                                    />
                                </div>

                                <div class="form-group">
                                    <label for="external-link">{"External Link"}</label>
                                    <input
                                        type="url"
                                        id="external-link"
                                        value={(*external_link).clone()}
                                        onchange={on_link_change}
                                        placeholder="https://your-event-page.com"
                                    />
                                    <small class="form-help">{"Link to registration page, meetup page, etc."}</small>
                                </div>
                            </div>

                            <div class="form-section">
                                <h2 class="section-title">{"📊 KPI Planning & Strategic Impact"}</h2>
                                <p class="section-description">{"Help us understand how this event contributes to Stellar's quarterly goals and measurable program outcomes."}</p>

                                <div class="form-group">
                                    <label>{"Strategic Focus Areas *"}</label>
                                    <small class="form-help">{"Select all areas this event contributes to. Every activity should connect to at least one strategic focus area."}</small>
                                    <div class="checkbox-grid">
                                        <label class="checkbox-label">
                                            <input
                                                type="checkbox"
                                                value="0"
                                                checked={strategic_focus_areas[0]}
                                                onchange={on_strategic_focus_change.clone()}
                                            />
                                            <span class="checkmark"></span>
                                            <div class="checkbox-content">
                                                <strong>{"Community Participation"}</strong>
                                                <small>{"Engaging and growing active ambassadors"}</small>
                                            </div>
                                        </label>
                                        <label class="checkbox-label">
                                            <input
                                                type="checkbox"
                                                value="1"
                                                checked={strategic_focus_areas[1]}
                                                onchange={on_strategic_focus_change.clone()}
                                            />
                                            <span class="checkmark"></span>
                                            <div class="checkbox-content">
                                                <strong>{"On-Chain Activity"}</strong>
                                                <small>{"Driving wallet creation, account usage, or on-chain activations"}</small>
                                            </div>
                                        </label>
                                        <label class="checkbox-label">
                                            <input
                                                type="checkbox"
                                                value="2"
                                                checked={strategic_focus_areas[2]}
                                                onchange={on_strategic_focus_change.clone()}
                                            />
                                            <span class="checkmark"></span>
                                            <div class="checkbox-content">
                                                <strong>{"SCF Referrals"}</strong>
                                                <small>{"Identifying and supporting high-quality companies and builders to apply to SCF"}</small>
                                            </div>
                                        </label>
                                        <label class="checkbox-label">
                                            <input
                                                type="checkbox"
                                                value="3"
                                                checked={strategic_focus_areas[3]}
                                                onchange={on_strategic_focus_change.clone()}
                                            />
                                            <span class="checkmark"></span>
                                            <div class="checkbox-content">
                                                <strong>{"Ecosystem Collaboration"}</strong>
                                                <small>{"Working with ecosystem partners to showcase Stellar-powered apps"}</small>
                                            </div>
                                        </label>
                                        <label class="checkbox-label">
                                            <input
                                                type="checkbox"
                                                value="4"
                                                checked={strategic_focus_areas[4]}
                                                onchange={on_strategic_focus_change.clone()}
                                            />
                                            <span class="checkmark"></span>
                                            <div class="checkbox-content">
                                                <strong>{"Developer Growth"}</strong>
                                                <small>{"Training, mentoring, or onboarding developers to build on Stellar"}</small>
                                            </div>
                                        </label>
                                    </div>
                                </div>

                                <div class="form-group">
                                    <label for="strategic-purpose">{"Strategic Purpose *"}</label>
                                    <textarea
                                        id="strategic-purpose"
                                        value={(*strategic_purpose).clone()}
                                        onchange={on_strategic_purpose_change}
                                        placeholder="Clearly explain how this event connects to KPIs and focus areas. What specific strategic purpose does it serve?"
                                        rows="3"
                                        required=true
                                    ></textarea>
                                    <small class="form-help">{"Every activity should have a clear strategic purpose."}</small>
                                </div>

                                <div class="form-group">
                                    <label for="target-audience">{"Target Audience *"}</label>
                                    <input
                                        type="text"
                                        id="target-audience"
                                        value={(*target_audience).clone()}
                                        onchange={on_target_audience_change}
                                        placeholder="e.g., Developers, Entrepreneurs, Students, Community members"
                                        required=true
                                    />
                                    <small class="form-help">{"Who specifically is this event targeting?"}</small>
                                </div>

                                <div class="form-group">
                                    <label>{"Primary KPI Estimates"}</label>
                                    <small class="form-help">{"Estimate your expected contributions to the three core program KPIs. Be explicit about your goals."}</small>
                                    <div class="kpi-grid">
                                        <div class="kpi-item">
                                            <label for="monthly-ambassadors">{"Monthly Active Ambassadors"}</label>
                                            <input
                                                type="number"
                                                id="monthly-ambassadors"
                                                value={(*monthly_active_ambassadors).clone()}
                                                onchange={on_monthly_ambassadors_change}
                                                placeholder="0"
                                                min="0"
                                            />
                                            <small>{"Number of active ambassadors engaged"}</small>
                                        </div>
                                        <div class="kpi-item">
                                            <label for="monthly-accounts">{"Monthly Active Accounts"}</label>
                                            <input
                                                type="number"
                                                id="monthly-accounts"
                                                value={(*monthly_active_accounts).clone()}
                                                onchange={on_monthly_accounts_change}
                                                placeholder="0"
                                                min="0"
                                            />
                                            <small>{"Wallets/accounts created as result of event"}</small>
                                        </div>
                                        <div class="kpi-item">
                                            <label for="scf-referrals">{"SCF Referrals"}</label>
                                            <input
                                                type="number"
                                                id="scf-referrals"
                                                value={(*scf_referrals).clone()}
                                                onchange={on_scf_referrals_change}
                                                placeholder="0"
                                                min="0"
                                            />
                                            <small>{"New builders/teams incubated or referred to SCF"}</small>
                                        </div>
                                    </div>
                                </div>

                                <div class="form-group">
                                    <label>{"Supporting Metrics"}</label>
                                    <small class="form-help">{"Additional metrics to track event success."}</small>
                                    <div class="metrics-grid">
                                        <div class="metric-item">
                                            <label for="content-produced">{"Content Produced"}</label>
                                            <input
                                                type="number"
                                                id="content-produced"
                                                value={(*content_produced).clone()}
                                                onchange={on_content_produced_change}
                                                placeholder="0"
                                                min="0"
                                            />
                                            <small>{"Articles, videos, tutorials, etc."}</small>
                                        </div>
                                        <div class="metric-item">
                                            <label for="expected-attendance">{"Expected Attendance"}</label>
                                            <input
                                                type="number"
                                                id="expected-attendance"
                                                value={(*expected_attendance).clone()}
                                                onchange={on_expected_attendance_change}
                                                placeholder="0"
                                                min="0"
                                            />
                                            <small>{"Estimated event attendance"}</small>
                                        </div>
                                        <div class="metric-item">
                                            <label for="social-growth">{"Social Growth Target"}</label>
                                            <input
                                                type="number"
                                                id="social-growth"
                                                value={(*social_growth_target).clone()}
                                                onchange={on_social_growth_change}
                                                placeholder="0"
                                                min="0"
                                            />
                                            <small>{"Expected social media growth/reach"}</small>
                                        </div>
                                    </div>
                                </div>

                                <div class="form-group">
                                    <label for="quarterly-goals">{"Quarterly Goals *"}</label>
                                    <textarea
                                        id="quarterly-goals"
                                        value={(*quarterly_goals).clone()}
                                        onchange={on_quarterly_goals_change}
                                        placeholder="How does this event fit into your chapter's quarterly strategic plan? What specific quarterly goals does it support?"
                                        rows="3"
                                        required=true
                                    ></textarea>
                                    <small class="form-help">{"Connect this event to your broader quarterly strategy."}</small>
                                </div>

                                <div class="form-group">
                                    <label for="success-metrics">{"Success Metrics & Test Plan"}</label>
                                    <textarea
                                        id="success-metrics"
                                        value={(*success_metrics).clone()}
                                        onchange={on_success_metrics_change}
                                        placeholder="How will you measure success? What specific metrics will you track? What is your test plan for validating the event achieved its goals?"
                                        rows="4"
                                    ></textarea>
                                    <small class="form-help">{"Define how you'll measure and validate the event's impact."}</small>
                                </div>
                            </div>

                            <button type="submit" class="submit-button">
                                {"Create Event"}
                            </button>
                        </form>
                    },
                    EventFormState::Loading => html! {
                        <div class="loading-state">
                            <div class="spinner"></div>
                            <h2>{"Creating your event..."}</h2>
                            <p>{"Please wait while we process your event."}</p>
                        </div>
                    },
                    EventFormState::Success(message) => html! {
                        <div class="success-state">
                            <div class="success-icon">{"✅"}</div>
                            <h2>{"Event Created Successfully!"}</h2>
                            <p class="success-message">{message}</p>
                            <div class="action-buttons">
                                <button class="primary-button" onclick={Callback::from(move |_| {
                                    web_sys::window().unwrap().location().set_href("/events").unwrap();
                                })}>
                                    {"View All Events"}
                                </button>
                                <button class="secondary-button" onclick={Callback::from(move |_| {
                                    state.set(EventFormState::Form);
                                })}>
                                    {"Create Another Event"}
                                </button>
                            </div>
                        </div>
                    },
                    EventFormState::Error(error) => html! {
                        <div class="error-state">
                            <div class="error-icon">{"❌"}</div>
                            <h2>{"Event Creation Failed"}</h2>
                            <p class="error-message">{error}</p>
                            <button class="retry-button" onclick={Callback::from(move |_| {
                                state.set(EventFormState::Form);
                            })}>
                                {"Try Again"}
                            </button>
                        </div>
                    }
                }}
            </div>

            <style>
                {include_str!("eventform.css")}
            </style>
        </div>
    }
}