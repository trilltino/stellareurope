use yew::prelude::*;
use crate::components::ui::{Input, Button, ButtonVariant};
use crate::hooks::{use_freighter, FreighterStatus};

#[derive(Properties, PartialEq)]
pub struct WalletInputProps {
    pub value: String,
    pub on_change: Callback<String>,
    #[prop_or_default]
    pub label: Option<String>,
    #[prop_or_default]
    pub error: Option<String>,
    #[prop_or(false)]
    pub required: bool,
    #[prop_or_default]
    pub help_text: Option<String>,
}

#[function_component(WalletInput)]
pub fn wallet_input(props: &WalletInputProps) -> Html {
    let freighter = use_freighter();

    // Auto-fill wallet address when Freighter connects
    {
        let on_change = props.on_change.clone();
        let public_key = freighter.get_public_key();
        use_effect_with_deps(move |public_key| {
            if let Some(key) = public_key {
                on_change.emit(key.clone());
            }
            || {}
        }, public_key);
    }

    let connect_freighter = {
        let connect = freighter.connect.clone();
        Callback::from(move |_| {
            connect.emit(());
        })
    };

    let button_text = match &*freighter.status {
        FreighterStatus::NotInstalled => "❌ Install Freighter",
        FreighterStatus::Disconnected => "🚀 Connect Freighter",
        FreighterStatus::Connecting => "⏳ Connecting...",
        FreighterStatus::Connected(_) => "✅ Connected",
        FreighterStatus::Error(_) => "❌ Connection Failed",
    };

    let help_text = match &*freighter.status {
        FreighterStatus::Connected(_) => Some("✅ Wallet connected via Freighter extension".to_string()),
        FreighterStatus::Error(error) => Some(format!("❌ {}", error)),
        _ => props.help_text.clone().or_else(|| {
            Some("Your Stellar public key - use Freighter extension or enter manually".to_string())
        }),
    };

    let button_variant = match &*freighter.status {
        FreighterStatus::Connected(_) => ButtonVariant::Success,
        FreighterStatus::Error(_) => ButtonVariant::Error,
        _ => ButtonVariant::Secondary,
    };

    html! {
        <div class="wallet-input-container">
            <Input
                label={props.label.clone()}
                value={props.value.clone()}
                on_change={props.on_change.clone()}
                placeholder="GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
                required={props.required}
                error={props.error.clone()}
                help_text={help_text}
                class={freighter.is_connected().then(|| "wallet-input--connected".to_string())}
            />

            <div class="wallet-input-actions">
                <Button
                    variant={button_variant}
                    onclick={connect_freighter}
                    disabled={freighter.is_connected() || freighter.is_connecting()}
                    loading={freighter.is_connecting()}
                >
                    {button_text}
                </Button>
            </div>
        </div>
    }
}