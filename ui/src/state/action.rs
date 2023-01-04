use std::rc::Weak;

use either::Either;
use warp::raygun::{Message, Reaction};
use wry::webview::WebView;

use super::{
    chats::Chat,
    identity::Identity,
    route::To,
    ui::{Theme, ToastNotification},
    State,
};

pub type Callback = Box<dyn Fn(&State, &Action)>;

// Define a new struct to represent a hook that listens for a specific action type.
pub struct ActionHook {
    pub action_type: Either<Action, Vec<Action>>,
    pub callback: Callback,
}

/// used exclusively by State::mutate
pub enum Action {
    // UI
    /// change ui::UI::popout_player, which determines if the media_view is anchored in place or moves around
    TogglePopout,
    EndAll,
    ToggleSilence,
    ToggleMute,
    SetOverlay(bool),
    AddToastNotification(ToastNotification),
    SetTheme(Theme),
    ClearTheme,
    // RemoveToastNotification,
    /// Toggles the display of media on the provided chat in the `State` struct.
    ToggleMedia(Chat),
    // Account
    /// Sets the ID for the user.
    SetId(Identity),
    AddWindow(Weak<WebView>),

    // Settings
    /// Sets the selected language.
    SetLanguage(String),

    // Routes
    /// Set the active route
    Navigate(To),
    // Requests
    /// Send a new friend request
    SendRequest(Identity),
    /// To be fired when a friend request you sent is accepted
    RequestAccepted(Identity),
    /// Cancel an outgoing request
    CancelRequest(Identity),

    /// Handle a new incoming friend request
    IncomingRequest(Identity),
    /// Accept an incoming friend request
    AcceptRequest(Identity),
    /// Deny a incoming friend request
    DenyRequest(Identity),

    // Friends
    RemoveFriend(Identity),
    Block(Identity),
    UnBlock(Identity),
    /// Handles the display of "favorite" chats
    Favorite(Chat),
    UnFavorite(Chat),
    /// Sets the active chat to a given chat
    ChatWith(Chat),
    /// Adds a chat to the sidebar
    AddToSidebar(Chat),
    /// Removes a chat from the sidebar, also removes the active chat if the chat being removed matches
    RemoveFromSidebar(Chat),
    /// Adds or removes a chat from the favorites page
    ToggleFavorite(Chat),

    // Messaging
    /// Records a new message and plays associated notifications
    NewMessage(Chat, Message),
    /// React to a given message by ID
    React(Chat, Message, Reaction),
    /// Reply to a given message by ID
    Reply(Chat, Message),
    /// Prep the UI for a message reply.
    StartReplying(Chat, Message),
    /// Clears the reply for a given chat
    CancelReply(Chat),
    /// Sends a message to the given chat
    Send(Chat, Message),
    ClearUnreads(Chat),
}

impl Action {
    pub fn compare_discriminant(&self, other: &Action) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}