use yew::prelude::*;
use wasm_bindgen::prelude::*;

#[derive(Clone, PartialEq)]
pub enum FreighterStatus {
    NotInstalled,
    Disconnected,
    Connecting,
    Connected(String), // public key
    Error(String),
}

#[derive(Clone)]
pub struct FreighterHandle {
    pub status: UseStateHandle<FreighterStatus>,
    pub connect: Callback<()>,
    pub disconnect: Callback<()>,
}

impl FreighterHandle {
    pub fn is_connected(&self) -> bool {
        matches!(*self.status, FreighterStatus::Connected(_))
    }

    pub fn is_connecting(&self) -> bool {
        matches!(*self.status, FreighterStatus::Connecting)
    }

    pub fn get_public_key(&self) -> Option<String> {
        match &*self.status {
            FreighterStatus::Connected(key) => Some(key.clone()),
            _ => None,
        }
    }

    pub fn get_error(&self) -> Option<String> {
        match &*self.status {
            FreighterStatus::Error(error) => Some(error.clone()),
            _ => None,
        }
    }
}

#[hook]
pub fn use_freighter() -> FreighterHandle {
    let status = use_state(|| FreighterStatus::Disconnected);

    let connect = {
        let status = status.clone();
        use_callback(move |_: ()| {
            let status = status.clone();
            status.set(FreighterStatus::Connecting);

            wasm_bindgen_futures::spawn_local(async move {
                match connect_to_freighter().await {
                    Ok(public_key) => {
                        status.set(FreighterStatus::Connected(public_key));
                    }
                    Err(error) => {
                        status.set(FreighterStatus::Error(error));
                    }
                }
            });
        }, ())
    };

    let disconnect = {
        let status = status.clone();
        use_callback(move |_: ()| {
            status.set(FreighterStatus::Disconnected);
        }, ())
    };

    FreighterHandle {
        status,
        connect,
        disconnect,
    }
}

async fn connect_to_freighter() -> Result<String, String> {
    let window = web_sys::window()
        .ok_or("No window object available")?;

    // Check if Freighter is available
    let freighter = js_sys::Reflect::get(&window, &JsValue::from_str("freighter"))
        .map_err(|_| "Freighter not found. Please install the Freighter extension.")?;

    if freighter.is_undefined() {
        return Err("Freighter not found. Please install the Freighter extension from the Chrome Web Store.".to_string());
    }

    // Call Freighter API to get public key
    let freighter_obj = freighter.dyn_into::<js_sys::Object>()
        .map_err(|_| "Invalid Freighter object")?;

    let get_public_key = js_sys::Reflect::get(&freighter_obj, &JsValue::from_str("getPublicKey"))
        .map_err(|_| "Freighter getPublicKey method not found")?;

    let get_public_key_fn = get_public_key.dyn_into::<js_sys::Function>()
        .map_err(|_| "Invalid getPublicKey function")?;

    let promise = get_public_key_fn.call0(&freighter_obj)
        .map_err(|_| "Failed to call getPublicKey")?;

    let promise = promise.dyn_into::<js_sys::Promise>()
        .map_err(|_| "Invalid promise returned")?;

    let future = wasm_bindgen_futures::JsFuture::from(promise);
    let public_key = future.await
        .map_err(|_| "Failed to get public key from Freighter. User may have rejected the request.")?;

    public_key.as_string()
        .ok_or("Invalid public key format".to_string())
}