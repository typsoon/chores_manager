use druid::{Color, Env, FontDescriptor, Key, Size};
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

// Day widget grid config
pub const CALENDAR_WEEKDAY_LABEL_COLOR: Key<Color> = Key::new("calendar_weekday_label_color");
pub const CALENDAR_DAY_NUMBER_LABEL_COLOR: Key<Color> = Key::new("calendar_day_number_label_color");
pub const CALENDAR_ACTIVE_MONTH_CONTAINER_COLOR: Key<Color> =
    Key::new("calendar_active_month_container_color");
pub const CALENDAR_INACTIVE_MONTH_CONTAINER_COLOR: Key<Color> =
    Key::new("calendar_inactive_month_container_color");

// Sub-window widget config
pub const SUB_WINDOW_WIDGET_WINDOW_SIZE: Key<Size> = Key::new("sub_window_widget_window_size");

// Scheduling widget config
pub const SCHEDULING_WIDGET_WEEKDAY_LABEL_FONT: Key<FontDescriptor> =
    Key::new("scheduling_widget_weekday_label_font");
pub const SCHEDULING_WIDGET_WEEKDAY_LABEL_WIDTH: Key<f64> =
    Key::new("scheduling_widget_weekday_label_width");
pub const SCHEDULING_WIDGET_WEEKDAY_LABEL_HEIGHT: Key<f64> =
    Key::new("scheduling_widget_weekday_label_height");

// Tabs setup
pub const TAB_CHORE_LABEL_WIDTH: Key<f64> = Key::new("chore_label_width");
pub const TAB_CHORE_DESCRIPTION_WIDTH: Key<f64> = Key::new("chore_description_width");

pub const TAB_PERSON_NAME_FIELD_WIDTH: Key<f64> = Key::new("person_name_field_width");
pub const TAB_ITEM_LIST_WIDTH: Key<f64> = Key::new("tab_item_width");

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

    env.set(CALENDAR_WEEKDAY_LABEL_COLOR, Color::rgb(0.6, 0.8, 0.9));
    env.set(CALENDAR_DAY_NUMBER_LABEL_COLOR, Color::rgb(0.4, 0.4, 0.4));
    env.set(
        CALENDAR_ACTIVE_MONTH_CONTAINER_COLOR,
        Color::rgb(0.7, 0.9, 1.0),
    );
    env.set(
        CALENDAR_INACTIVE_MONTH_CONTAINER_COLOR,
        Color::rgb(0.8, 0.8, 0.8),
    );

    env.set(SUB_WINDOW_WIDGET_WINDOW_SIZE, (500., 500.));

    //
    env.set(
        SCHEDULING_WIDGET_WEEKDAY_LABEL_FONT,
        FontDescriptor::new(druid::FontFamily::SYSTEM_UI).with_size(12.),
    );
    env.set(SCHEDULING_WIDGET_WEEKDAY_LABEL_WIDTH, 85.);
    env.set(SCHEDULING_WIDGET_WEEKDAY_LABEL_HEIGHT, 20.);

    //
    env.set(TAB_CHORE_LABEL_WIDTH, 400.0);
    env.set(TAB_CHORE_DESCRIPTION_WIDTH, 300.0);

    env.set(TAB_PERSON_NAME_FIELD_WIDTH, 200.0);
    env.set(TAB_ITEM_LIST_WIDTH, 750.);

    env.set(
        LOGIN_SCREEN_TEXT_FIELD_FONT,
        FontDescriptor::new(druid::FontFamily::SYSTEM_UI)
            .with_weight(druid::FontWeight::BOLD)
            .with_size(env.get(LOGIN_SCREEN_FONT_SIZE)),
    );
}
