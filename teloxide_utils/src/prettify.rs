use teloxide::{
    prelude::*,
    types::{
        Chat, ChatJoinRequest, ChatMember, ChatMigration, Forward, ForwardedFrom, MediaKind,
        MessageKind, UpdateKind, User,
    },
};

pub fn prettify_chat(chat: &Chat) -> String {
    if chat.is_private() {
        if let Some(last_name) = chat.last_name() {
            format!(
                "{} {} ({})",
                chat.first_name().unwrap(),
                last_name,
                chat.id.0
            )
        } else {
            format!("{} ({})", chat.first_name().unwrap(), chat.id.0)
        }
    } else {
        format!("{} ({})", chat.title().unwrap(), chat.id.0)
    }
}

pub fn prettify_user(user: &User) -> String {
    if let Some(last_name) = &user.last_name {
        format!("{} {} ({})", user.first_name, last_name, user.id.0)
    } else {
        format!("{} ({})", user.first_name, user.id.0)
    }
}

pub fn prettify_forward(fwd: &Forward) -> String {
    let mut pretty = String::from("Forwarded from");

    match &fwd.from {
        ForwardedFrom::User(user) => pretty = format!("{} {}", pretty, prettify_user(&user)),
        ForwardedFrom::Chat(chat) => pretty = format!("{} {}", pretty, prettify_chat(&chat)),
        ForwardedFrom::SenderName(name) => pretty = format!("{} {}", pretty, name),
    };

    pretty
}

pub fn prettify_message(msg: &Message, update_type: &str) -> String {
    let mut pretty = if let Some(thread_id) = msg.thread_id {
        format!("{} ({}, {})", update_type, msg.id.0, thread_id)
    } else {
        format!("{} ({})", update_type, msg.id.0)
    };
    pretty = format!("{}\nTimestamp: {}", pretty, msg.date.timestamp());
    pretty = format!("{}\nChat: {}", pretty, prettify_chat(&msg.chat));

    match &msg.kind {
        MessageKind::Common(common) => {
            pretty = format!("{}\nType: Common", pretty);

            if let Some(user) = &common.from {
                pretty = format!("{}\nFrom: {}", pretty, prettify_user(&user));
            }
            if let Some(sender_chat) = &common.sender_chat {
                pretty = format!("{}\nFrom: {}", pretty, prettify_chat(&sender_chat));
            }
            if let Some(author) = &common.author_signature {
                pretty = format!("{}\nAuthor: {}", pretty, author);
            }

            if common.has_protected_content {
                pretty = format!(
                    "{}\nProtected Content: {}",
                    pretty, common.has_protected_content
                );
            }

            if common.is_automatic_forward {
                pretty = format!(
                    "{}\nIs Automatic Forward: {}",
                    pretty, common.is_automatic_forward
                );
            }
            if let Some(forward) = &common.forward {
                pretty = format!("{}\nForwarded From: {}", pretty, prettify_forward(&forward));
            }

            if let Some(reply_to) = &common.reply_to_message {
                pretty = format!("{}\nReply To: {}", pretty, reply_to.id.0);
            }

            match &common.media_kind {
                MediaKind::Animation(animation) => {
                    pretty = format!("{}\nMedia: Animation", pretty);
                    if let Some(caption) = &animation.caption {
                        pretty = format!("{}\nCaption: {}", pretty, caption);
                    }
                    if animation.has_media_spoiler {
                        pretty = format!("{}\nSpoiler: {}", pretty, animation.has_media_spoiler);
                    }
                }
                MediaKind::Audio(audio) => {
                    pretty = format!("{}\nMedia: Audio", pretty);
                    if let Some(caption) = &audio.caption {
                        pretty = format!("{}\nCaption: {}", pretty, caption);
                    }
                }
                MediaKind::Contact(_) => {
                    pretty = format!("{}\nMedia: Contact", pretty);
                }
                MediaKind::Document(document) => {
                    pretty = format!("{}\nMedia: Document", pretty);
                    if let Some(caption) = &document.caption {
                        pretty = format!("{}\nCaption: {}", pretty, caption);
                    }
                }
                MediaKind::Game(_) => {
                    pretty = format!("{}\nMedia: Game", pretty);
                }
                MediaKind::Venue(_) => {
                    pretty = format!("{}\nMedia: Venue", pretty);
                }
                MediaKind::Location(_) => {
                    pretty = format!("{}\nMedia: Location", pretty);
                }
                MediaKind::Photo(photo) => {
                    pretty = format!("{}\nMedia: Photo", pretty);
                    if let Some(caption) = &photo.caption {
                        pretty = format!("{}\nCaption: {}", pretty, caption);
                    }
                }
                MediaKind::Poll(_) => {
                    pretty = format!("{}\nMedia: Poll", pretty);
                }
                MediaKind::Sticker(sticker) => {
                    pretty = format!("{}\nMedia: Sticker", pretty);
                    pretty = format!("{}\nFormat: {:?}", pretty, sticker.sticker.format);
                    pretty = format!("{}\nType: {:?}", pretty, sticker.sticker.type_());
                }
                MediaKind::Text(text) => {
                    pretty = format!("{}\nText: {}", pretty, text.text);
                }
                MediaKind::Video(video) => {
                    pretty = format!("{}\nMedia: Video", pretty);
                    if let Some(caption) = &video.caption {
                        pretty = format!("{}\nCaption: {}", pretty, caption);
                    }
                }
                MediaKind::VideoNote(_) => {
                    pretty = format!("{}\nMedia: Video Note", pretty);
                }
                MediaKind::Voice(voice) => {
                    pretty = format!("{}\nMedia: Voice", pretty);
                    if let Some(caption) = &voice.caption {
                        pretty = format!("{}\nCaption: {}", pretty, caption);
                    }
                }
                MediaKind::Migration(migration) => {
                    pretty = format!("{}\nMedia: Migration", pretty);
                    match &migration {
                        ChatMigration::To { chat_id } => {
                            pretty = format!("{}\nTo: {}", pretty, chat_id.0);
                        }
                        ChatMigration::From { chat_id } => {
                            pretty = format!("{}\nFrom: {}", pretty, chat_id.0);
                        }
                    }
                }
            };
        }
        MessageKind::NewChatMembers(mem) => {
            pretty = format!(
                "{}\nType: New Chat Members\nUsers: {:?}",
                pretty,
                mem.new_chat_members
                    .iter()
                    .map(|user| prettify_user(&user))
                    .collect::<Vec<_>>()
            );
        }
        MessageKind::LeftChatMember(mem) => {
            pretty = format!(
                "{}\nType: Left Chat Member\nUser: {}",
                pretty,
                prettify_user(&mem.left_chat_member),
            );
        }
        MessageKind::NewChatTitle(title) => {
            pretty = format!(
                "{}\nType: New Chat Title\nTitle: {}",
                pretty, title.new_chat_title
            );
        }
        MessageKind::NewChatPhoto(_) => {
            pretty = format!("{}\nType: New Chat Photo", pretty,);
        }
        MessageKind::DeleteChatPhoto(_) => {
            pretty = format!("{}\nType: Delete Chat Photo", pretty,);
        }
        MessageKind::GroupChatCreated(_) => {
            pretty = format!("{}\nType: Group Created", pretty,);
        }
        MessageKind::SupergroupChatCreated(_) => {
            pretty = format!("{}\nType: Supergroup Created", pretty,);
        }
        MessageKind::ChannelChatCreated(_) => {
            pretty = format!("{}\nType: Channel Created", pretty,);
        }
        MessageKind::MessageAutoDeleteTimerChanged(changed) => {
            pretty = format!(
                "{}\nType: Message Auto Delete Timer Changed\nNew Time: {}",
                pretty,
                changed
                    .message_auto_delete_timer_changed
                    .message_auto_delete_time
            );
        }
        MessageKind::Pinned(pinned) => {
            pretty = format!(
                "{}\nType: Pinned Message\nMessage ID: {}",
                pretty, pinned.pinned.id
            );
        }
        MessageKind::Invoice(invoice) => {
            pretty = format!(
                "{}\nType: Invoice\nTitle: {}\nDescription: {}\nAmount: {}\nCurrency: {:?}\nStart Parameter: {}",
                pretty,
                invoice.invoice.title,
                invoice.invoice.description,
                invoice.invoice.total_amount,
                invoice.invoice.currency,
                invoice.invoice.start_parameter,
            );
        }
        MessageKind::SuccessfulPayment(payment) => {
            pretty = format!(
                "{}\nType: Successful Payment\nAmount: {}\nCurrency: {:?}\nInvoice Payload: {}",
                pretty,
                payment.successful_payment.total_amount,
                payment.successful_payment.currency,
                payment.successful_payment.invoice_payload,
            );
        }
        MessageKind::ConnectedWebsite(website) => {
            pretty = format!(
                "{}\nType: Connected Website\nWebsite: {}",
                pretty, website.connected_website,
            );
        }
        MessageKind::WriteAccessAllowed(_) => {
            pretty = format!("{}\nType: Write Access Allowed", pretty);
        }
        MessageKind::PassportData(_) => {
            pretty = format!("{}\nType: Passport Data", pretty);
        }
        MessageKind::Dice(dice) => {
            pretty = format!("{}\nType: Dice\nValue: {}", pretty, dice.dice.value);
        }
        MessageKind::ProximityAlertTriggered(trigger) => {
            pretty = format!(
                "{}\nType: Proximity Alert Triggered\nTraveler: {}\nWatcher: {}\nDistance: {}",
                pretty,
                prettify_user(&trigger.proximity_alert_triggered.traveler),
                prettify_user(&trigger.proximity_alert_triggered.watcher),
                trigger.proximity_alert_triggered.distance,
            );
        }
        MessageKind::ForumTopicCreated(created) => {
            pretty = format!(
                "{}\nType: Forum Topic Created\nName: {:?}\nEmoji: {:?}",
                pretty,
                created.forum_topic_created.name,
                created.forum_topic_created.icon_custom_emoji_id,
            );
        }
        MessageKind::ForumTopicEdited(edited) => {
            pretty = format!(
                "{}\nType: Forum Topic Edited\nName: {:?}\nEmoji: {:?}",
                pretty,
                edited.forum_topic_edited.name,
                edited.forum_topic_edited.icon_custom_emoji_id,
            );
        }
        MessageKind::ForumTopicClosed(_) => {
            pretty = format!("{}\nType: Forum Topic Closed", pretty);
        }
        MessageKind::ForumTopicReopened(_) => {
            pretty = format!("{}\nType: Forum Topic Reopened", pretty);
        }
        MessageKind::GeneralForumTopicHidden(_) => {
            pretty = format!("{}\nType: Forum Topic Hidden", pretty);
        }
        MessageKind::GeneralForumTopicUnhidden(_) => {
            pretty = format!("{}\nType: Forum Topic Unhidden", pretty);
        }
        MessageKind::VideoChatScheduled(scheduled) => {
            pretty = format!(
                "{}\nType: Video Chat Scheduled ({})",
                pretty,
                scheduled.video_chat_scheduled.start_date.timestamp()
            );
        }
        MessageKind::VideoChatStarted(_) => {
            pretty = format!("{}\nType: Video Chat Started", pretty);
        }
        MessageKind::VideoChatEnded(_) => {
            pretty = format!("{}\nType: Video Chat Ended", pretty);
        }
        MessageKind::VideoChatParticipantsInvited(invited) => {
            if let Some(users) = &invited.video_chat_participants_invited.users {
                pretty = format!(
                    "{}\nType: Video Chat Participants Invited\nUsers: {:?}",
                    pretty,
                    users
                        .iter()
                        .map(|user| prettify_user(user))
                        .collect::<Vec<_>>()
                );
            } else {
                pretty = format!("{}\nType: Video Chat Participants Invited", pretty);
            }
        }
        MessageKind::WebAppData(data) => {
            pretty = format!(
                "{}\nType: Web App Data\nData: {}\nButton Text: {}",
                pretty, data.web_app_data.data, data.web_app_data.button_text
            );
        }
    };

    pretty
}

pub fn prettify_inline_query(query: &InlineQuery) -> String {
    let mut pretty = format!("Inline Query ({})", query.id);

    pretty = format!("{}\nFrom: {}", pretty, prettify_user(&query.from));
    if let Some(chat_type) = &query.chat_type {
        pretty = format!("{}\nChat Type: {:?}", pretty, chat_type);
    }
    pretty = format!("{}\nQuery: {}", pretty, query.query);
    pretty = format!("{}\nOffset: {}", pretty, query.offset);

    pretty
}

pub fn prettify_chosen_inline_result(res: &ChosenInlineResult) -> String {
    let mut pretty = if let Some(inline_id) = &res.inline_message_id {
        format!("Chosen Inline Result ({})", inline_id)
    } else {
        String::from("Chosen Inline Result")
    };

    pretty = format!("{}\nFrom: {}", pretty, prettify_user(&res.from));
    pretty = format!("{}\nQuery: {}", pretty, res.query);
    pretty = format!("{}\nResult ID: {}", pretty, res.result_id);

    pretty
}

pub fn prettify_callback_query(query: &CallbackQuery) -> String {
    let mut pretty = format!("Callback Query ({})", query.id);

    if let Some(inline_id) = &query.inline_message_id {
        pretty = format!("{}\nInline Message ID: {}", pretty, inline_id);
    }

    pretty = format!("{}\nFrom: {}", pretty, prettify_user(&query.from));

    if let Some(data) = &query.data {
        pretty = format!("{}\nData: {}", pretty, data);
    } else if let Some(game) = &query.game_short_name {
        pretty = format!("{}\nGame: {}", pretty, game);
    }

    if let Some(message) = &query.message {
        pretty = format!(
            "{}\nMessage: {}",
            pretty,
            prettify_message(message, "Callback Message").replace('\n', "\n         ")
        );
    }

    pretty
}

pub fn prettify_shipping_query(query: &ShippingQuery) -> String {
    let mut pretty = format!("Shipping Query ({})", query.id);

    pretty = format!("{}\nFrom: {}", pretty, prettify_user(&query.from));
    pretty = format!("{}\nInvoice Payload: {}", pretty, query.invoice_payload);

    pretty
}

pub fn prettify_pre_checkout_query(query: &PreCheckoutQuery) -> String {
    let mut pretty = format!("Pre Checkout Query ({})", query.id);

    pretty = format!("{}\nFrom: {}", pretty, prettify_user(&query.from));
    pretty = format!(
        "{}\nAmount: {} {:?}",
        pretty, query.total_amount, query.currency
    );
    pretty = format!("{}\nInvoice Payload: {}", pretty, query.invoice_payload);

    if let Some(shipping_option) = &query.shipping_option_id {
        pretty = format!("{}\nShipping: {}", pretty, shipping_option);
    }

    pretty
}

pub fn prettify_poll(poll: &Poll) -> String {
    let mut pretty = format!("Poll ({})", poll.id);

    pretty = format!("{}\nPoll Type: {:?}", pretty, poll.poll_type);

    pretty = format!("{}\nQuestion: {}", pretty, poll.question);

    pretty = format!("{}\nIs Anonymous: {}", pretty, poll.is_anonymous);
    pretty = format!("{}\nIs Closed: {}", pretty, poll.is_closed);
    pretty = format!(
        "{}\nAllows Multiple: {}",
        pretty, poll.allows_multiple_answers
    );

    pretty
}

pub fn prettify_poll_answer(ans: &PollAnswer) -> String {
    let mut pretty = format!("Poll Answer ({})", ans.poll_id);

    pretty = format!("{}\nUser: {}", pretty, prettify_user(&ans.user));

    pretty
}

pub fn prettify_chat_member(member: &ChatMember) -> String {
    format!("{}: {:?}", prettify_user(&member.user), member.kind)
}

pub fn prettify_chat_member_updated(update: &ChatMemberUpdated, mine: bool) -> String {
    let mut pretty = if mine {
        String::from("My Chat Member Updated")
    } else {
        String::from("Chat Member Updated")
    };

    pretty = format!("{}\nTime: {}", pretty, update.date.timestamp());
    pretty = format!("{}\nChat: {}", pretty, prettify_chat(&update.chat));
    pretty = format!("{}\nUpdated By: {}", pretty, prettify_user(&update.from));

    if let Some(invite) = &update.invite_link {
        pretty = format!("{}\nInvite Link: {}", pretty, invite.invite_link);
    }

    pretty = format!(
        "{}\nOld Membership: {}",
        pretty,
        prettify_chat_member(&update.old_chat_member)
    );
    pretty = format!(
        "{}\nNew Membership: {}",
        pretty,
        prettify_chat_member(&update.new_chat_member)
    );

    pretty
}

pub fn prettify_chat_join_request(request: &ChatJoinRequest) -> String {
    let mut pretty = String::from("Chat Join Request");

    pretty = format!("{}\nTime: {}", pretty, request.date.timestamp());
    pretty = format!("{}\nChat: {}", pretty, prettify_chat(&request.chat));
    pretty = format!("{}\nUser: {}", pretty, prettify_user(&request.from));
    if let Some(invite) = &request.invite_link {
        pretty = format!("{}\nInvite: {}", pretty, invite.invite_link);
    }

    pretty
}

pub fn prettify_update(update: &Update) -> String {
    let mut pretty = format!("Update ({})", update.id);

    match &update.kind {
        UpdateKind::Message(msg) => {
            pretty = pretty + "\n" + prettify_message(msg, "New Message").as_str();
        }
        UpdateKind::EditedMessage(msg) => {
            pretty = pretty + "\n" + prettify_message(msg, "Edited Message").as_str();
        }
        UpdateKind::ChannelPost(post) => {
            pretty = pretty + "\n" + prettify_message(post, "New Channel Post").as_str();
        }
        UpdateKind::EditedChannelPost(post) => {
            pretty = pretty + "\n" + prettify_message(post, "Edited Channel Post").as_str();
        }
        UpdateKind::InlineQuery(query) => {
            pretty = pretty + "\n" + prettify_inline_query(&query).as_str();
        }
        UpdateKind::ChosenInlineResult(res) => {
            pretty = pretty + "\n" + prettify_chosen_inline_result(&res).as_str();
        }
        UpdateKind::CallbackQuery(query) => {
            pretty = pretty + "\n" + prettify_callback_query(&query).as_str();
        }
        UpdateKind::ShippingQuery(query) => {
            pretty = pretty + "\n" + prettify_shipping_query(&query).as_str();
        }
        UpdateKind::PreCheckoutQuery(query) => {
            pretty = pretty + "\n" + prettify_pre_checkout_query(&query).as_str();
        }
        UpdateKind::Poll(poll) => {
            pretty = pretty + "\n" + prettify_poll(&poll).as_str();
        }
        UpdateKind::PollAnswer(ans) => {
            pretty = pretty + "\n" + prettify_poll_answer(&ans).as_str();
        }
        UpdateKind::MyChatMember(mem) => {
            pretty = pretty + "\n" + prettify_chat_member_updated(&mem, true).as_str();
        }
        UpdateKind::ChatMember(mem) => {
            pretty = pretty + "\n" + prettify_chat_member_updated(&mem, false).as_str();
        }
        UpdateKind::ChatJoinRequest(req) => {
            pretty = pretty + "\n" + prettify_chat_join_request(&req).as_str();
        }
        UpdateKind::Error(err) => {
            pretty = pretty + "\nError while parsing:\n";
            pretty = pretty + serde_json::to_string(&err).unwrap().as_str();
        }
    };

    pretty
}
