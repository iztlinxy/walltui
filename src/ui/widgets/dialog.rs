#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmAction {
    DeleteGalleryItem,
    BulkDeleteGallery,
}

#[derive(Debug, Clone)]
pub struct ConfirmDialog {
    pub message: String,
    pub action: ConfirmAction,
}

impl ConfirmDialog {
    pub fn new(message: impl Into<String>, action: ConfirmAction) -> Self {
        Self {
            message: message.into(),
            action,
        }
    }
}
