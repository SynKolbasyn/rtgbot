use serde::{Deserialize, Serialize};

use crate::types::{
    user::User,
    chat::Chat,
    message_origin::MessageOrigin,
    external_reply_info::ExternalReplyInfo,
    text_quote::TextQuote,
    story::Story,
    message_entity::MessageEntity,
    link_preview_options::LinkPreviewOptions,
    animation::Animation,
    audio::Audio,
    document::Document,
    photo_size::PhotoSize,
    sticker::Sticker,
    video::Video,
    video_note::VideoNote,
    voice::Voice,
    contact::Contact,
    dice::Dice,
    game::Game,
    poll::Poll,
    venue::Venue,
    location::Location,
    message_auto_delete_timer_changed::MessageAutoDeleteTimerChanged,
    invoice::Invoice,
    successful_payment::SuccessfulPayment,
    users_shared::UsersShared,
    chat_shared::ChatShared,
    write_access_allowed::WriteAccessAllowed,
    passport_data::PassportData,
    proximity_alert_triggered::ProximityAlertTriggered,
    chat_boost_added::ChatBoostAdded,
    chat_background::ChatBackground,
    forum_topic_created::ForumTopicCreated,
    forum_topic_edited::ForumTopicEdited,
    forum_topic_closed::ForumTopicClosed,
    forum_topic_reopened::ForumTopicReopened,
    general_forum_topic_hidden::GeneralForumTopicHidden,
    general_forum_topic_unhidden::GeneralForumTopicUnhidden,
    giveaway_created::GiveawayCreated,
    giveaway::Giveaway,
    giveaway_winners::GiveawayWinners,
    giveaway_completed::GiveawayCompleted,
    video_chat_scheduled::VideoChatScheduled,
    video_chat_started::VideoChatStarted,
    video_chat_ended::VideoChatEnded,
    video_chat_participants_invited::VideoChatParticipantsInvited,
    web_app_data::WebAppData,
    inline_keyboard_markup::InlineKeyboardMarkup,
};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MaybeInaccessibleMessage {
    Message(Message),
    InaccessibleMessage(InaccessibleMessage)
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    message_id: i64,
    message_thread_id: Option<i64>,
    from: Option<User>,
    sender_chat: Option<Chat>,
    sender_boost_count: Option<i64>,
    sender_business_bot: Option<User>,
    date: i64,
    business_connection_id: Option<String>,
    chat: Chat,
    forward_origin: Option<MessageOrigin>,
    is_topic_message: Option<bool>,
    is_automatic_forward: Option<bool>,
    reply_to_message: Option<Box<Message>>,
    external_reply: Option<ExternalReplyInfo>,
    quote: Option<TextQuote>,
    reply_to_story: Option<Story>,
    via_bot: Option<User>,
    edit_date: Option<i64>,
    has_protected_content: Option<bool>,
    is_from_offline: Option<bool>,
    media_group_id: Option<String>,
    author_signature: Option<String>,
    text: Option<String>,
    entities: Option<Vec<MessageEntity>>,
    link_preview_options: Option<LinkPreviewOptions>,
    animation: Option<Animation>,
    audio: Option<Audio>,
    document: Option<Document>,
    photo: Option<Vec<PhotoSize>>,
    sticker: Option<Sticker>,
    story: Option<Story>,
    video: Option<Video>,
    video_note: Option<VideoNote>,
    voice: Option<Voice>,
    caption: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    has_media_spoiler: Option<bool>,
    contact: Option<Contact>,
    dice: Option<Dice>,
    game: Option<Game>,
    poll: Option<Poll>,
    venue: Option<Venue>,
    location: Option<Location>,
    new_chat_members: Option<Vec<User>>,
    left_chat_member: Option<User>,
    new_chat_title: Option<String>,
    new_chat_photo: Option<Vec<PhotoSize>>,
    delete_chat_photo: Option<bool>,
    group_chat_created: Option<bool>,
    supergroup_chat_created: Option<bool>,
    channel_chat_created: Option<bool>,
    message_auto_delete_timer_changed: Option<MessageAutoDeleteTimerChanged>,
    migrate_to_chat_id: Option<i64>,
    migrate_from_chat_id: Option<i64>,
    pinned_message: Option<Box<MaybeInaccessibleMessage>>,
    invoice: Option<Invoice>,
    successful_payment: Option<SuccessfulPayment>,
    users_shared: Option<UsersShared>,
    chat_shared: Option<ChatShared>,
    connected_website: Option<String>,
    write_access_allowed: Option<WriteAccessAllowed>,
    passport_data: Option<PassportData>,
    proximity_alert_triggered: Option<ProximityAlertTriggered>,
    boost_added: Option<ChatBoostAdded>,
    chat_background_set: Option<ChatBackground>,
    forum_topic_created: Option<ForumTopicCreated>,
    forum_topic_edited: Option<ForumTopicEdited>,
    forum_topic_closed: Option<ForumTopicClosed>,
    forum_topic_reopened: Option<ForumTopicReopened>,
    general_forum_topic_hidden: Option<GeneralForumTopicHidden>,
    general_forum_topic_unhidden: Option<GeneralForumTopicUnhidden>,
    giveaway_created: Option<GiveawayCreated>,
    giveaway: Option<Giveaway>,
    giveaway_winners: Option<GiveawayWinners>,
    giveaway_completed: Option<GiveawayCompleted>,
    video_chat_scheduled: Option<VideoChatScheduled>,
    video_chat_started: Option<VideoChatStarted>,
    video_chat_ended: Option<VideoChatEnded>,
    video_chat_participants_invited: Option<VideoChatParticipantsInvited>,
    web_app_data: Option<WebAppData>,
    reply_markup: Option<InlineKeyboardMarkup>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InaccessibleMessage {
    chat: Chat,
    message_id: i64,
    date: i64,
}


impl Message {
    
}
