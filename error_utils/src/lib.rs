pub trait BaseError: std::fmt::Display + std::fmt::Debug {
    #[track_caller]
    fn with_description_and_error<S: ToString>(description: S, error: Option<String>) -> Self;
    #[track_caller]
    fn add_location(self) -> Self;
}
