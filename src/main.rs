use cursive::views::{TextView, HideableView, ResizedView};
use cursive::view::{Selector, Resizable, Nameable};
use cursive::{Cursive, CursiveExt};

// Config file paths.
const PATH_COLOR_SCHEME: &str = include_str!("colors.toml");

fn m_init_hello_open(a: &mut Cursive) {
    let m = TextView::new("h to open hello!").full_screen();
    a.add_layer(HideableView::new(m).with_name("hello"));
}

fn main() {
    let mut app = Cursive::new();
    m_init_hello_open(&mut app);

    app.load_toml(PATH_COLOR_SCHEME).unwrap();

    app.add_global_callback('`', Cursive::toggle_debug_console);
    app.add_global_callback('q', |s| s.quit());
    app.add_global_callback('t', |s| {
        s.call_on(
            &Selector::Name("hello"),
            |view: &mut HideableView<ResizedView<TextView>>| {
                view.set_visible(!view.is_visible());
            }
        );
    });

    app.run();
}
