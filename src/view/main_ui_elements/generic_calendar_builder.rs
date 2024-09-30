use druid::im::{vector, Vector};
use druid::lens::Constant;
use druid::widget::{Flex, List, ListIter};
use druid::Lens;
use druid::{Data, Widget, WidgetExt};
use std::mem;
use std::rc::Rc;

struct ToWeeksLens;

impl ToWeeksLens {
    fn get_as_weeks<D: Data, LD: ListIter<D>>(data: &LD) -> Vector<Vector<D>> {
        let mut weeks = vector![];
        let mut evaluated_week = vector![];

        data.for_each(|day_data, index| {
            if index % 7 == 0 && index != 0 {
                weeks.push_back(mem::take(&mut evaluated_week));
            }
            evaluated_week.push_back(day_data.clone());
        });
        weeks.push_back(evaluated_week);
        weeks
    }
}

impl<D: Data, LD: ListIter<D>> Lens<LD, Vector<Vector<D>>> for ToWeeksLens {
    fn with<V, F: FnOnce(&Vector<Vector<D>>) -> V>(&self, data: &LD, f: F) -> V {
        f(&Self::get_as_weeks(data))
    }

    fn with_mut<V, F: FnOnce(&mut Vector<Vector<D>>) -> V>(&self, data: &mut LD, f: F) -> V {
        f(&mut Self::get_as_weeks(data))
    }
}

pub fn build_calendar<WS, D, WD, LD>(
    weekday_label_widget_builder: impl Fn() -> WS + 'static,
    day_widget_builder: impl Fn() -> WD + 'static,
) -> impl Widget<LD>
where
    WS: Widget<String> + 'static,
    D: Data,
    WD: Widget<D> + 'static,
    LD: ListIter<D>,
{
    let weekdays = Vector::from(
        vector!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]
            .into_iter()
            .map(String::from)
            .collect::<Vector<_>>(),
    );

    let cloned_day_widget_builder = Rc::new(day_widget_builder);
    Flex::column()
        .with_child(
            List::new(move || weekday_label_widget_builder())
                .horizontal()
                .lens(Constant(weekdays)),
        )
        .with_child(
            List::new(move || {
                let temp_day_widget_builder = cloned_day_widget_builder.clone();
                List::new(move || temp_day_widget_builder()).horizontal()
            })
            .lens(ToWeeksLens),
        )
}
