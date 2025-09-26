use yew::prelude::*;
use shared::dto::{SignUpRequest, SignUpResponse, UserType};
use crate::services::api;
use crate::components::{Card, Input, WalletInput, FormSection, Button, ButtonVariant, ButtonSize};
use crate::hooks::use_form;

#[derive(PartialEq, Clone)]
pub enum SignupState {
    Form,
    Loading,
    Success(SignUpResponse),
    Error(String),
}

#[function_component(SignupPage)]
pub fn signup_page() -> Html {
    let state = use_state(|| SignupState::Form);
    let form = use_form();
    let user_type = use_state(|| UserType::Ambassador);

    let on_user_type_change = {
        let user_type = user_type.clone();
        Callback::from(move |value: String| {
            let selected_type = if value == "ChapterLead" {
                UserType::ChapterLead
            } else {
                UserType::Ambassador
            };
            user_type.set(selected_type);
        })
    };

    let on_submit = {
        let state = state.clone();
        let form = form.clone();
        let user_type = user_type.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            if !form.validate_all() {
                return;
            }

            let request = SignUpRequest {
                username: form.get_value("username"),
                email: form.get_value("email"),
                wallet_address: form.get_value("wallet_address"),
                user_type: (*user_type).clone(),
                organization: {
                    let org = form.get_value("organization");
                    if org.is_empty() { None } else { Some(org) }
                },
                bio: {
                    let bio = form.get_value("bio");
                    if bio.is_empty() { None } else { Some(bio) }
                },
            };

            state.set(SignupState::Loading);

            let state_clone = state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match api::signup(request).await {
                    Ok(response) => {
                        state_clone.set(SignupState::Success(response));
                    }
                    Err(e) => {
                        state_clone.set(SignupState::Error(format!("Signup failed: {}", e)));
                    }
                }
            });
        })
    };

    html! {
        <div class="signup-container">
            <Card
                title="Join Stellar Europe"
                subtitle="Become part of Europe's leading Stellar community"
                elevated=true
            >
                {match &*state {
                    SignupState::Form => html! {
                        <form onsubmit={on_submit}>
                            <FormSection title="Personal Information" icon="👤">
                                <Input
                                    label="Username"
                                    value={form.get_value("username")}
                                    on_change={form.get_callback("username")}
                                    placeholder="Enter your username"
                                    required=true
                                    error={form.get_error("username")}
                                />

                                <Input
                                    label="Email Address"
                                    input_type="email"
                                    value={form.get_value("email")}
                                    on_change={form.get_callback("email")}
                                    placeholder="your.email@example.com"
                                    required=true
                                    error={form.get_error("email")}
                                />

                                <WalletInput
                                    label="Stellar Wallet Address"
                                    value={form.get_value("wallet_address")}
                                    on_change={form.get_callback("wallet_address")}
                                    required=true
                                    error={form.get_error("wallet_address")}
                                />
                            </FormSection>

                            <FormSection title="Role Selection" icon="🎯">
                                <div class="role-selection">
                                    <div class="role-option">
                                        <input
                                            type="radio"
                                            id="ambassador"
                                            name="user_type"
                                            value="Ambassador"
                                            checked={*user_type == UserType::Ambassador}
                                            onchange={on_user_type_change.clone()}
                                        />
                                        <label for="ambassador" class="role-label ambassador-role">
                                            <div class="role-header">
                                                <span class="role-icon">{"🌟"}</span>
                                                <span class="role-name">{"Ambassador"}</span>
                                            </div>
                                            <p class="role-description">
                                                {"Represent Stellar in your local community, organize meetups, and help educate others about blockchain technology."}
                                            </p>
                                        </label>
                                    </div>

                                    <div class="role-option">
                                        <input
                                            type="radio"
                                            id="chapter-lead"
                                            name="user_type"
                                            value="ChapterLead"
                                            checked={*user_type == UserType::ChapterLead}
                                            onchange={on_user_type_change}
                                        />
                                        <label for="chapter-lead" class="role-label chapter-lead-role">
                                            <div class="role-header">
                                                <span class="role-icon">{"🏛️"}</span>
                                                <span class="role-name">{"Chapter Lead"}</span>
                                            </div>
                                            <p class="role-description">
                                                {"Lead regional chapters, coordinate large-scale events, and drive strategic initiatives across multiple cities."}
                                            </p>
                                        </label>
                                    </div>
                                </div>
                            </FormSection>

                            <FormSection title="Additional Information" icon="📝" bordered=false>
                                <Input
                                    label="Organization (Optional)"
                                    value={form.get_value("organization")}
                                    on_change={form.get_callback("organization")}
                                    placeholder="Company, University, or Organization"
                                />

                                <Input
                                    label="Bio (Optional)"
                                    value={form.get_value("bio")}
                                    on_change={form.get_callback("bio")}
                                    placeholder="Tell us about yourself, your interests in blockchain, and why you want to join..."
                                />
                            </FormSection>

                            <Button
                                button_type="submit"
                                variant={ButtonVariant::Primary}
                                size={ButtonSize::Large}
                                full_width=true
                            >
                                {"Join Stellar Europe"}
                            </Button>
                        </form>
                    },
                    SignupState::Loading => html! {
                        <div class="loading-state">
                            <div class="spinner"></div>
                            <h2>{"Creating your account..."}</h2>
                            <p>{"Please wait while we process your registration."}</p>
                        </div>
                    },
                    SignupState::Success(response) => html! {
                        <div class="success-state">
                            <div class="success-icon">{"✅"}</div>
                            <h2>{"Welcome to Stellar Europe!"}</h2>
                            <p class="success-message">{&response.message}</p>
                            <div class="user-info">
                                <h3>{"Your Profile"}</h3>
                                <p><strong>{"Username:"}</strong> {&response.user.username}</p>
                                <p><strong>{"Role:"}</strong> {&response.user.user_type}</p>
                                <p><strong>{"Email:"}</strong> {&response.user.email}</p>
                            </div>
                            <Button
                                variant={ButtonVariant::Primary}
                                size={ButtonSize::Medium}
                                full_width=true
                                onclick={Callback::from(move |_| {
                                    web_sys::window().unwrap().location().set_href("/").unwrap();
                                })}
                            >
                                {"Continue to Dashboard"}
                            </Button>
                        </div>
                    },
                    SignupState::Error(error) => html! {
                        <div class="error-state">
                            <div class="error-icon">{"❌"}</div>
                            <h2>{"Signup Failed"}</h2>
                            <p class="error-message">{error}</p>
                            <Button
                                variant={ButtonVariant::Secondary}
                                size={ButtonSize::Medium}
                                onclick={Callback::from(move |_| {
                                    state.set(SignupState::Form);
                                })}
                            >
                                {"Try Again"}
                            </Button>
                        </div>
                    }
                }}
            </Card>
        </div>
    }
}