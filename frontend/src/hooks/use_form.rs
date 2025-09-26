use yew::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, PartialEq)]
pub struct FormField {
    pub value: String,
    pub error: Option<String>,
    pub touched: bool,
}

impl FormField {
    pub fn new() -> Self {
        Self {
            value: String::new(),
            error: None,
            touched: false,
        }
    }

    pub fn with_value(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            error: None,
            touched: false,
        }
    }
}

impl Default for FormField {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct FormHandle {
    fields: UseStateHandle<HashMap<String, FormField>>,
    validators: Rc<HashMap<String, Box<dyn Fn(&str) -> Option<String>>>>,
}

impl FormHandle {
    pub fn get_field(&self, name: &str) -> FormField {
        self.fields.get(name).cloned().unwrap_or_default()
    }

    pub fn get_value(&self, name: &str) -> String {
        self.get_field(name).value
    }

    pub fn get_error(&self, name: &str) -> Option<String> {
        self.get_field(name).error
    }

    pub fn set_value(&self, name: &str, value: String) {
        let mut fields = (*self.fields).clone();
        let mut field = fields.get(name).cloned().unwrap_or_default();
        field.value = value;
        field.touched = true;

        // Run validation if validator exists
        if let Some(validator) = self.validators.get(name) {
            field.error = validator(&field.value);
        }

        fields.insert(name.to_string(), field);
        self.fields.set(fields);
    }

    pub fn set_error(&self, name: &str, error: Option<String>) {
        let mut fields = (*self.fields).clone();
        let mut field = fields.get(name)
        .cloned()
        .unwrap_or_default();
        field.error = error;
        fields.insert(name.to_string(), field);
        self.fields.set(fields);
    }

    pub fn get_callback(&self, name: &str) -> Callback<String> {
        let handle = self.clone();
        let field_name = name.to_string();
        Callback::from(move |value: String| {
            handle.set_value(&field_name, value);
        })
    }

    pub fn validate_all(&self) -> bool {
        let mut fields = (*self.fields).clone();
        let mut all_valid = true;

        for (name, validator) in self.validators.iter() {
            let field = fields.get(name).cloned().unwrap_or_default();
            if let Some(error) = validator(&field.value) {
                let mut updated_field = field;
                updated_field.error = Some(error);
                fields.insert(name.clone(), updated_field);
                all_valid = false;
            }
        }

        self.fields.set(fields);
        all_valid
    }

    pub fn reset(&self) {
        self.fields.set(HashMap::new());
    }

    pub fn has_errors(&self) -> bool {
        self.fields.values().any(|field| field.error.is_some())
    }

    pub fn is_field_valid(&self, name: &str) -> bool {
        self.get_field(name).error.is_none()
    }
}

pub fn use_form() -> FormHandle {
    let fields = use_state(|| HashMap::new());
    let validators = Rc::new(HashMap::new());

    FormHandle {
        fields,
        validators,
    }
}

pub fn use_form_with_validators(
    validators: HashMap<String, Box<dyn Fn(&str) -> Option<String>>>
) -> FormHandle {
    let fields = use_state(HashMap::new);
    let validators = Rc::new(validators);

    FormHandle {
        fields,
        validators,
    }
}