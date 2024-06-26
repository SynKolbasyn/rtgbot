use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WriteAccessAllowed {
    from_request: Option<bool>,
    web_app_name: Option<String>,
    from_attachment_menu: Option<bool>,
}
