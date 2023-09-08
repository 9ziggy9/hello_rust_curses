use cursive::traits::Nameable;
use cursive::views::{TextView, HideableView, NamedView};
use cursive::view::Selector;
use cursive::{Cursive, CursiveExt};

// Type aliases
type NamedHideable = NamedView<HideableView<TextView>>;

// Config file paths.
const PATH_COLOR_SCHEME: &str = include_str!("colors.toml");

fn m_init_hello_open() -> NamedHideable {
    let m = TextView::new("h to open hello!");
    return HideableView::new(m).with_name("hello");
}

fn main() {
    let hello = m_init_hello_open();
    let mut app = Cursive::new();

    app.load_toml(PATH_COLOR_SCHEME).unwrap();
    app.add_layer(hello);

    app.add_global_callback('`', Cursive::toggle_debug_console);
    app.add_global_callback('q', |s| s.quit());
    app.add_global_callback('t', |s| {
        s.call_on(
            &Selector::Name("hello"),
            |view: &mut HideableView<TextView>| {
                view.set_visible(!view.is_visible());
            }
        );
    });
    app.run();
}
