use crate::error_handling::ErrorWithTeloxideUpdate;

use teloxide::{
    payloads::EditMessageTextSetters,
    prelude::*,
    types::{ChatId, InlineKeyboardMarkup, InputFile, MessageId, ReplyMarkup, True},
};

fn prepare_base_send_message_request<B: Requester, S: Into<String>>(
    bot: &B,
    chat_id: ChatId,
    text: S,
    reply_to_message_id_opt: Option<MessageId>,
) -> B::SendMessage {
    let mut req = bot.send_message(chat_id, text);

    if let Some(reply_to_message_id) = reply_to_message_id_opt {
        req = req.reply_to_message_id(reply_to_message_id);
    }

    req = req
        .disable_web_page_preview(true)
        .allow_sending_without_reply(true);

    req
}

fn prepare_base_send_photo_request<B: Requester, S: Into<String>>(
    bot: &B,
    chat_id: ChatId,
    photo: InputFile,
    caption: Option<S>,
    reply_to_message_id_opt: Option<MessageId>,
) -> B::SendPhoto {
    let mut req = bot.send_photo(chat_id, photo);

    if let Some(reply_to_message_id) = reply_to_message_id_opt {
        req = req.reply_to_message_id(reply_to_message_id);
    }
    if let Some(caption) = caption {
        req = req.caption(caption);
    }

    req = req.allow_sending_without_reply(true);

    req
}

fn prepare_base_edit_message_request<B: Requester, S: Into<String>>(
    bot: &B,
    chat_id: ChatId,
    msg_id: MessageId,
    text: S,
) -> B::EditMessageText {
    let mut req = bot.edit_message_text(chat_id, msg_id, text);

    req = req.disable_web_page_preview(true);

    req
}

fn prepare_base_callback_answer_request<B: Requester>(
    bot: &B,
    callback_query_id: &str,
    show_alert: bool,
) -> B::AnswerCallbackQuery {
    let mut req = bot.answer_callback_query(callback_query_id);

    req = req.show_alert(show_alert);

    req
}

pub async fn send_text<B: Requester, S: Into<String>, E: ErrorWithTeloxideUpdate>(
    bot: &B,
    chat_id: ChatId,
    text: S,
    reply_to_message_id_opt: Option<MessageId>,
) -> Result<Message, E> {
    let req = prepare_base_send_message_request(bot, chat_id, text, reply_to_message_id_opt);

    req.await.map_err(|err| {
        E::with_description_and_error(
            if reply_to_message_id_opt.is_some() {
                "failed to reply text"
            } else {
                "failed to send text"
            },
            Some(err.to_string()),
        )
    })
}

pub async fn send_photo<B: Requester, S: Into<String>, E: ErrorWithTeloxideUpdate>(
    bot: &B,
    chat_id: ChatId,
    photo: InputFile,
    caption: Option<S>,
    reply_to_message_id_opt: Option<MessageId>,
) -> Result<Message, E> {
    let req =
        prepare_base_send_photo_request(bot, chat_id, photo, caption, reply_to_message_id_opt);

    req.await
        .map_err(|err| E::with_description_and_error("failed to send photo", Some(err.to_string())))
}

pub async fn send_text_with_reply_markup<
    B: Requester,
    S: Into<String>,
    R: Into<ReplyMarkup>,
    E: ErrorWithTeloxideUpdate,
>(
    bot: &B,
    chat_id: ChatId,
    text: S,
    reply_to_message_id_opt: Option<MessageId>,
    reply_markup: R,
) -> Result<Message, E> {
    let mut req = prepare_base_send_message_request(bot, chat_id, text, reply_to_message_id_opt);

    req = req.reply_markup(reply_markup.into());

    req.await.map_err(|err| {
        E::with_description_and_error(
            if reply_to_message_id_opt.is_some() {
                "failed to reply with reply markup"
            } else {
                "failed to send text with reply markup"
            },
            Some(err.to_string()),
        )
    })
}

pub async fn answer_callback_text<B: Requester, S: Into<String>, E: ErrorWithTeloxideUpdate>(
    bot: &B,
    callback_query_id: &str,
    show_alert: bool,
    text: S,
) -> Result<True, E> {
    let mut req = prepare_base_callback_answer_request(bot, callback_query_id, show_alert);

    req = req.text(text);

    req.await.map_err(|err| {
        E::with_description_and_error("failed to answer callback query", Some(err.to_string()))
    })
}

pub async fn edit_message_inline_keyboard<
    B: Requester,
    S: Into<String>,
    E: ErrorWithTeloxideUpdate,
>(
    bot: &B,
    chat_id: ChatId,
    msg_id: MessageId,
    text: S,
    keyboard: InlineKeyboardMarkup,
) -> Result<Message, E> {
    let mut req = prepare_base_edit_message_request(bot, chat_id, msg_id, text);

    req = req.reply_markup(keyboard);

    req.await.map_err(|err| {
        E::with_description_and_error("failed to edit reply markup", Some(err.to_string()))
    })
}
