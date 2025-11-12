#[derive(Debug, Clone, poise::ChoiceParameter)]
pub enum ReminderKind {
    #[name = "Medicine"] Medicine,
    #[name = "Food"] Food,
    #[name = "Other"] Other,
}