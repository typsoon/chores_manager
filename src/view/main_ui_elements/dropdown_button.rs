use druid::im::Vector;
use druid::lens::LensExt;
use druid::widget::{Controller, Either, Flex, Label, List, Scope};
use druid::{Color, Data, EventCtx, Lens, Selector, Target, Widget, WidgetExt, WidgetId};
use std::fmt::Display;

const VARIANT_CHOSEN: Selector<usize> = Selector::new("variant_chosen");

type OptionsAndSelected<T> = (T, Vector<T>);

#[derive(Clone, Data, Lens)]
struct DropdownData<T: Data + Display + Default + Clone> {
    selected: usize,
    options_and_selected: OptionsAndSelected<T>,
    dropped: bool,
}

impl<T: Data + Display + Default + Clone> DropdownData<T> {
    pub fn new(options_and_selected: (T, Vector<T>)) -> Self {
        Self { selected: 0, options_and_selected, dropped: false }
    }
}

// impl<T: Data + Display + Default + Clone> DropdownData<T> {
//     pub fn new(options: Vector<T>) -> Self {
//         Self {
//             selected: 0,
//             options_and_selected: options,
//             dropped: false,
//         }
//     }
// }

struct DropdownController;

impl<T: Data + Display + Default + Clone, W: Widget<DropdownData<T>>> Controller<DropdownData<T>, W>
    for DropdownController
{
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &druid::Event,
        data: &mut DropdownData<T>,
        env: &druid::Env,
    ) {
        if let druid::Event::Command(cmd) = event {
            if let Some(chosen_idx) = cmd.get(VARIANT_CHOSEN) {
                data.selected = *chosen_idx;
                data.dropped = false;
                data.options_and_selected.0 = data.options_and_selected.1.get(data.selected).cloned().unwrap_or_default();
                ctx.set_handled();
                ctx.request_update();
            }
        }
        child.event(ctx, event, data, env);
    }
}

pub fn get_dropdown_button<T: Data + Display + Default + Clone>() -> impl Widget<(T, Vector<T>)> {
    let controlling_widget_id = WidgetId::next();

    Scope::from_lens(
        DropdownData::new,
        DropdownData::options_and_selected,
        Flex::column()
            .with_child(
                Label::new(|item: &OptionsAndSelected<T>, _env: &_| {
                    format!(
                        "{}",
                        item.0
                        // item.options_and_selected.get(item.selected).cloned().unwrap_or_default()
                    )
                })
                .with_text_color(Color::BLACK)
                .padding((5., 5., 5., 5.))
                .background(Color::rgb(0.7, 0.9, 1.0))
                .lens(DropdownData::options_and_selected)
                .on_click(
                    |_ctx: &mut EventCtx, data: &mut DropdownData<T>, _env: &_| {
                        data.dropped = !data.dropped;
                    },
                ),
            )
            .with_child(Either::new(
                |data: &DropdownData<T>, _| data.dropped,
                List::new(move || {
                    Label::new(move |item: &(usize, T), _env: &_| format!("{}", item.1))
                        .padding((5., 5., 5., 5.))
                        .on_click(move |ctx: &mut EventCtx, data: &mut _, _env| {
                            ctx.submit_command(
                                VARIANT_CHOSEN
                                    .with(data.0)
                                    .to(Target::Widget(controlling_widget_id)),
                            );
                        })
                })
                .lens(DropdownData::options_and_selected.map(
                    |data: &OptionsAndSelected<T>| data.1.iter().cloned().enumerate().collect(),
                    |data: &mut OptionsAndSelected<T>, new_data: Vector<(usize, T)>| {
                        data.1 = new_data
                            .iter()
                            .map(|(_, item)| item.clone())
                            .collect::<Vector<_>>();
                    },
                ))
                .scroll()
                .fix_height(100.0),
                Flex::column(),
            ))
            .with_id(controlling_widget_id)
            .controller(DropdownController),
    )
}
