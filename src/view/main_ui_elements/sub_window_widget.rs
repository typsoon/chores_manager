use crate::view::view_types::wrappers::{ChoresDataKeyVal, FullChoreDataWrapper};
use druid::widget::{BackgroundBrush, Label, List, Painter};
use druid::{Color, Widget, WidgetExt};

const CHORES_DESC_TEXT_SIZE: f64 = 20.0;

pub fn build_sub_window_widget() -> impl Widget<ChoresDataKeyVal> {
    List::new(|| {
        let chore_box_painter =
            Painter::new(|ctx: &mut _, data: &FullChoreDataWrapper, env: &_| {
                let mut brush = BackgroundBrush::Color(if data.was_completed() {
                    Color::GREEN
                } else {
                    Color::RED
                });
                brush.paint(ctx, data, env)
            });

        Label::new(|item: &FullChoreDataWrapper, _env: &_| {
            format!("{}\t{}\n", item.chore_name(), item.person_name())
        })
        .with_text_size(CHORES_DESC_TEXT_SIZE)
        .background(chore_box_painter)
        .padding((0., 5., 0., 0.))
    })
    .scroll()
    .fix_height(500.)
    .lens(ChoresDataKeyVal::chores)
    .center()
}
