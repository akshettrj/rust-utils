use error_utils::BaseError;

use teloxide::{
    prelude::*,
    utils::{command::ParseError, html::escape as html_escape},
};

pub trait ErrorWithTeloxideUpdate: BaseError {
    #[track_caller]
    fn with_description_error_and_update<S: ToString>(
        description: S,
        error: Option<String>,
        update: Update,
    ) -> Self;
    #[track_caller]
    fn add_update(self, update: Update) -> Self;
    #[track_caller]
    fn add_location_and_update(self, update: Update) -> Self;
}

pub async fn handle_command_parse_error<B: Requester, E: ErrorWithTeloxideUpdate>(
    err: ParseError,
    bot: &B,
    msg: Message,
    update: Update,
) -> Result<(), E> {
    let response = match err {
        ParseError::TooFewArguments {
            expected,
            found,
            message,
        } => Some(format!(
            "Too few arguments:\n\nExpected: {}\nFound: {}\nMessage: <code>{}</code>",
            expected,
            found,
            html_escape(&message)
        )),
        ParseError::TooManyArguments {
            expected,
            found,
            message,
        } => Some(format!(
            "Too many arguments:\n\nExpected: {}\nFound: {}\nMessage: <code>{}</code>",
            expected,
            found,
            html_escape(&message)
        )),
        ParseError::IncorrectFormat(format_error) => Some(format!(
            "Incorrect format:\n\n<code>{}</code>",
            html_escape(&format_error.to_string())
        )),
        ParseError::UnknownCommand(_) => None,
        ParseError::WrongBotName(_) => None,
        ParseError::Custom(custom) => Some(custom.to_string()),
    };

    if let Some(response) = response {
        crate::api_wrappers::send_text(bot, msg.chat.id, response, Some(msg.id))
            .await
            .map_err(|err: E| err.add_location_and_update(update))?;
    }

    Ok(())
}
