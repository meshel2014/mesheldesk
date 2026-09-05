use hbb_common::config::{self, keys};

pub const APP_NAME: &str = "MeshelDesk";
pub const ID_SERVER: &str = "remote.meshel.cn";
pub const RELAY_SERVER: &str = "remote.meshel.cn:21117";
pub const SERVER_KEY: &str = "oVvhm7iAfi5UZ3j9++b7ysrtMEvyKBz+iLgEzSzrzgE=";
pub const HIDE_INSTALL_CARD: &str = "hide-install-card";

pub fn apply() {
    *config::APP_NAME.write().unwrap() = APP_NAME.to_owned();
    *config::PROD_RENDEZVOUS_SERVER.write().unwrap() = ID_SERVER.to_owned();
    *config::EXE_RENDEZVOUS_SERVER.write().unwrap() = String::new();

    let mut settings = config::OVERWRITE_SETTINGS.write().unwrap();
    settings.insert(
        keys::OPTION_CUSTOM_RENDEZVOUS_SERVER.to_owned(),
        ID_SERVER.to_owned(),
    );
    settings.insert(
        keys::OPTION_RELAY_SERVER.to_owned(),
        RELAY_SERVER.to_owned(),
    );
    settings.insert(keys::OPTION_KEY.to_owned(), SERVER_KEY.to_owned());
    settings.insert(keys::OPTION_API_SERVER.to_owned(), String::new());
    settings.insert(keys::OPTION_ALLOW_AUTO_UPDATE.to_owned(), "N".to_owned());
    drop(settings);

    config::BUILTIN_SETTINGS
        .write()
        .unwrap()
        .insert(keys::OPTION_HIDE_SERVER_SETTINGS.to_owned(), "Y".to_owned());
    config::BUILTIN_SETTINGS
        .write()
        .unwrap()
        .insert(HIDE_INSTALL_CARD.to_owned(), "Y".to_owned());
}
