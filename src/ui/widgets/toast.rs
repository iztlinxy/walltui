use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct Toast {
    pub message: String,
    pub created_at: Instant,
    pub duration: Duration,
}

impl Toast {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            created_at: Instant::now(),
            duration: Duration::from_secs(3),
        }
    }

    pub fn expired(&self) -> bool {
        self.created_at.elapsed() >= self.duration
    }
}

#[derive(Debug, Default)]
pub struct ToastManager {
    pub toast: Option<Toast>,
}

impl ToastManager {
    pub fn show(&mut self, message: impl Into<String>) {
        self.toast = Some(Toast::new(message));
    }

    pub fn tick(&mut self) {
        if let Some(toast) = &self.toast {
            if toast.expired() {
                self.toast = None;
            }
        }
    }

    pub fn message(&self) -> Option<&str> {
        self.toast.as_ref().map(|t| t.message.as_str())
    }
}
