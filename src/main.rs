use cursive::views::{
    TextView, HideableView,
    ResizedView, Button, SelectView
};
use cursive::view::{Selector, Resizable, Nameable};
use cursive::Cursive;

// Config file paths.
const PATH_COLOR_SCHEME: &str = include_str!("colors.toml");

fn views_init_help(a: &mut Cursive) {
    let m = TextView::new("h to open hello!").full_screen();
    a.add_fullscreen_layer(HideableView::new(m).with_name("help"));
}

fn views_init_panes(a: &mut Cursive) {
    let select = SelectView::<String>::new()
        .on_submit(|s: &mut Cursive, name: &str| println!("Value: {}", name))
        .item("Hello", "World".to_string())
        .item("Goodbye", "Moon".to_string())
        .fixed_size((10,5));
    let panes = cursive::views::LinearLayout::horizontal()
        .child(select)
        .child(Button::new("World", |s| s.quit()));
    a.add_fullscreen_layer(panes.full_screen());
}

fn app_init_keys(a: &mut Cursive) {
    a.add_global_callback('`', Cursive::toggle_debug_console);
    a.add_global_callback('q', |s| s.quit());
    a.add_global_callback('t', |s| {
        s.call_on(
            &Selector::Name("help"),
            |view: &mut HideableView<ResizedView<TextView>>| {
                view.set_visible(!view.is_visible());
            }
        );
    });
}

fn main() {
    let mut app = cursive::default();
    views_init_help(&mut app);
    views_init_panes(&mut app);

    app.load_toml(PATH_COLOR_SCHEME).unwrap();

    app_init_keys(&mut app);

    app.run();
}
