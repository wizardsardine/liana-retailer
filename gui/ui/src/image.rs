use crate::widget::Svg;
use iced::{widget::svg::Handle, window::icon};

const LIANA_APP_ICON: &[u8] = include_bytes!("../static/logos/liana-app-icon.png");
const LIANA_LOGO_GREY: &[u8] = include_bytes!("../static/logos/LIANA_SYMBOL_Gray.svg");
const LIANA_BRAND_GREY: &[u8] = include_bytes!("../static/logos/LIANA_BRAND_Gray.svg");
const WIZARDSARDINE_LETTERING: &[u8] = include_bytes!("../static/logos/logo-wizardsardine.svg");

const RETAILER_ICON: &[u8] = include_bytes!("../static/logos/21st-capital-app-icon.png");
const RETAILER_LOGO: &[u8] = include_bytes!("../static/logos/21ST_CAPITAL_White.svg");

pub fn liana_app_icon() -> icon::Icon {
    icon::from_file_data(LIANA_APP_ICON, None).unwrap()
}

pub fn liana_grey_logo() -> Svg {
    let h = Handle::from_memory(LIANA_LOGO_GREY.to_vec());
    Svg::new(h)
}

pub fn retailer_logo() -> Svg {
    let h = Handle::from_memory(RETAILER_LOGO.to_vec());
    Svg::new(h)
}

pub fn retailer_app_icon() -> icon::Icon {
    icon::from_file_data(RETAILER_ICON, None).unwrap()
}

pub fn liana_brand_grey() -> Svg {
    let h = Handle::from_memory(LIANA_BRAND_GREY.to_vec());
    Svg::new(h)
}

pub fn wizardsardine() -> Svg {
    let h = Handle::from_memory(WIZARDSARDINE_LETTERING.to_vec());
    Svg::new(h)
}

const CREATE_NEW_WALLET_ICON: &[u8] = include_bytes!("../static/icons/blueprint.svg");

pub fn create_new_wallet_icon() -> Svg {
    let h = Handle::from_memory(CREATE_NEW_WALLET_ICON.to_vec());
    Svg::new(h)
}

const PARTICIPATE_IN_NEW_WALLET_ICON: &[u8] = include_bytes!("../static/icons/discussion.svg");

pub fn participate_in_new_wallet_icon() -> Svg {
    let h = Handle::from_memory(PARTICIPATE_IN_NEW_WALLET_ICON.to_vec());
    Svg::new(h)
}

const RESTORE_WALLET_ICON: &[u8] = include_bytes!("../static/icons/syncdata.svg");

pub fn restore_wallet_icon() -> Svg {
    let h = Handle::from_memory(RESTORE_WALLET_ICON.to_vec());
    Svg::new(h)
}

const SUCCESS_MARK_ICON: &[u8] = include_bytes!("../static/icons/success-mark.svg");

pub fn success_mark_icon() -> Svg {
    let h = Handle::from_memory(SUCCESS_MARK_ICON.to_vec());
    Svg::new(h)
}

const KEY_MARK_ICON: &[u8] = include_bytes!("../static/icons/key-mark.svg");

pub fn key_mark_icon() -> Svg {
    let h = Handle::from_memory(KEY_MARK_ICON.to_vec());
    Svg::new(h)
}
