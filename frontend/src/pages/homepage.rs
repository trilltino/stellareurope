use yew::prelude::*;
use yew_router::prelude::*;
use crate::routing::Route;

#[function_component(HomePage)]
pub fn homepage() -> Html {
    html! {
        <div class="home-container">
            <div class="hero-section">
                <div class="hero-content">
                    <h1 class="hero-title">
                        {"Welcome to "}
                        <span class="brand-highlight">{"Stellar Europe"}</span>
                    </h1>
                    <p class="hero-subtitle">
                        {"Connecting blockchain innovators across Europe through the power of Stellar"}
                    </p>
                    <p class="hero-description">
                        {"Join our community of Ambassadors and Chapter Leads working together to advance Stellar adoption,
                         organize impactful events, and build the future of decentralized finance in Europe."}
                    </p>
                    <div class="cta-buttons">
                        <Link<Route> to={Route::Signup} classes="cta-primary">
                            {"Join as Ambassador"}
                        </Link<Route>>
                        <Link<Route> to={Route::Signup} classes="cta-secondary">
                            {"Become Chapter Lead"}
                        </Link<Route>>
                    </div>
                </div>
            </div>

            <div class="features-section">
                <div class="features-container">
                    <h2 class="section-title">{"What We Do"}</h2>
                    <div class="features-grid">
                        <div class="feature-card">
                            <div class="feature-icon">{"🌟"}</div>
                            <h3>{"Ambassador Program"}</h3>
                            <p>{"Represent Stellar in your community, organize meetups, and educate others about blockchain technology."}</p>
                        </div>
                        <div class="feature-card">
                            <div class="feature-icon">{"🏛️"}</div>
                            <h3>{"Chapter Leadership"}</h3>
                            <p>{"Lead regional chapters, coordinate large-scale events, and drive strategic initiatives across Europe."}</p>
                        </div>
                        <div class="feature-card">
                            <div class="feature-icon">{"📅"}</div>
                            <h3>{"Event Management"}</h3>
                            <p>{"Plan and execute workshops, conferences, hackathons, and community gatherings to grow the ecosystem."}</p>
                        </div>
                        <div class="feature-card">
                            <div class="feature-icon">{"🤝"}</div>
                            <h3>{"Community Building"}</h3>
                            <p>{"Foster connections between developers, entrepreneurs, and blockchain enthusiasts across the continent."}</p>
                        </div>
                    </div>
                </div>
            </div>

            <div class="stats-section">
                <div class="stats-container">
                    <div class="stat-item">
                        <div class="stat-number">{"50+"}</div>
                        <div class="stat-label">{"Cities"}</div>
                    </div>
                    <div class="stat-item">
                        <div class="stat-number">{"200+"}</div>
                        <div class="stat-label">{"Ambassadors"}</div>
                    </div>
                    <div class="stat-item">
                        <div class="stat-number">{"1000+"}</div>
                        <div class="stat-label">{"Events"}</div>
                    </div>
                    <div class="stat-item">
                        <div class="stat-number">{"25"}</div>
                        <div class="stat-label">{"Countries"}</div>
                    </div>
                </div>
            </div>

            <style>
                {include_str!("homepage.css")}
            </style>
        </div>
    }
}