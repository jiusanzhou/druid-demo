use druid::widget::{Align, Button, Column, Label, Padding};
use druid::{AppLauncher, LocalizedString, Widget, WindowDesc};

static I18N_BASE: &str = "../resources/i18n";

fn main() {
    let main_window = WindowDesc::new(app_builder);
    let data = 0_u32;
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
        .expect("launch failed");
}

fn app_builder() -> impl Widget<u32> {
    // let l10nm = L10nManager::new(vec!["demo"], I18N_BASE);
    
    // The label text will be computed dynamically based on the current locale and count
    let text =
        LocalizedString::new("hello-counter").with_arg("count", |data: &u32, _env| (*data).into());
    let label = Label::new(text);
    let button = Button::new("increment", |_ctx, data, _env| *data += 1);

    let mut col = Column::new();
    col.add_child(label, 1.0);
    col.add_child(button, 1.0);
    col
}