use druid::{Env, FontDescriptor, Key};
// Login screen config
pub const LOGIN_SCREEN_VERTICAL_WIDGET_SPACING: Key<f64> =
    Key::new("login_screen_vertical_widget_spacing");
pub const LOGIN_SCREEN_TEXT_BOX_WIDTH: Key<f64> = Key::new("login_screen_text_box_width");
pub const LOGIN_SCREEN_FONT_SIZE: Key<f64> = Key::new("login_screen_font_size");

pub const LOGIN_SCREEN_TEXT_FIELD_FONT: Key<FontDescriptor> =
    Key::new("login_screen_text_field_font");

// Calendar config
pub const DAY_NAME_CELL_HEIGHT: Key<f64> = Key::new("day_name_cell_height");
pub const DAY_NAME_CELL_WIDTH: Key<f64> = Key::new("day_name_cell_width");

pub const DAY_NAME_FONT_SIZE: f64 = 24.0;
pub const CALENDAR_ITEM_WIDTH: Key<f64> = Key::new("calendar_item_width");

pub const CALENDAR_ITEM_HEIGHT: Key<f64> = Key::new("calendar_item_height");
pub const CALENDAR_CHORE_ITEM_WIDTH: Key<f64> = Key::new("calendar_chore_item_width");

pub const CALENDAR_CHORE_ITEM_HEIGHT: Key<f64> = Key::new("calendar_chore_item_height");

pub const CHORES_DESC_TEXT_SIZE: Key<f64> = Key::new("chores_desc_text_size");

pub const CHORE_LIST_HEIGHT: Key<f64> = Key::new("chore_list_height");

pub fn configure_env(env: &mut Env) {
    env.set(LOGIN_SCREEN_VERTICAL_WIDGET_SPACING, 10.0);
    env.set(LOGIN_SCREEN_TEXT_BOX_WIDTH, 200.0);
    env.set(LOGIN_SCREEN_FONT_SIZE, 18.0);

    env.set(DAY_NAME_CELL_WIDTH, 200.0);
    env.set(DAY_NAME_CELL_HEIGHT, 60.0);

    env.set(CALENDAR_ITEM_WIDTH, 200.0);
    env.set(CALENDAR_ITEM_HEIGHT, 100.0);

    env.set(CALENDAR_CHORE_ITEM_WIDTH, 85.);
    env.set(CALENDAR_CHORE_ITEM_HEIGHT, 20.);

    env.set(CHORES_DESC_TEXT_SIZE, 10.0);

    env.set(CHORE_LIST_HEIGHT, 72.5);

    env.set(
        LOGIN_SCREEN_TEXT_FIELD_FONT,
        FontDescriptor::new(druid::FontFamily::SYSTEM_UI)
            .with_weight(druid::FontWeight::BOLD)
            .with_size(env.get(LOGIN_SCREEN_FONT_SIZE)),
    );
}
