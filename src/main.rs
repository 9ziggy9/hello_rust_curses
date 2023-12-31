use std::collections::HashMap;
use cursive::views::{
    TextView, HideableView, Dialog,
    ResizedView, SelectView
};
use cursive::view::{Selector, Resizable, Nameable};
use cursive::Cursive;
use cursive::align::{HAlign, VAlign};
use serde::{Deserialize};
use std::{fs, io};
use std::fs::File;
use std::io::Read;

// JSON types
#[derive(Debug, Deserialize)]
struct Card {
    name: String,
    content: String,
}

// Config file paths.
const PATH_COLOR_SCHEME: &str = include_str!("colors.toml");

fn list_files() -> io::Result<Vec<String>> {
    let current_dir = std::env::current_dir()?;
    let entries = fs::read_dir(current_dir)?;
    let file_names: Vec<String> = entries
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.is_file() {
                path.file_name()?.to_str().map(|s| s.to_owned())
            } else {
                None
            }
        })
        .collect();
    Ok(file_names)
}

// File extension should include .
fn filter_files_by_ext(files: &Vec<String>, ext: &str) -> Vec<String> {
    return files
        .iter()
        .filter(|f| f.ends_with(ext))
        .cloned()
        .collect();
}

fn read_json_to_cards(path: &str) -> (&str, Vec<Card>) {
    let card_category = path.split(".").collect::<Vec<&str>>()[0];
    let file = File::open(path).expect("Failed to open file.");
    let mut raw_json = String::new();
    let _ = file
        .take(20_000)
        .read_to_string(&mut raw_json)
        .expect("Failed to read file.");
    return (
        card_category,
        serde_json::from_str::<Vec<Card>>(&raw_json)
            .expect("Failed to parse JSON.")
    );
}

fn views_init_help(a: &mut Cursive) {
    let m = TextView::new("h to open hello!").full_screen();
    a.add_fullscreen_layer(HideableView::new(m).with_name("help"));
}

fn views_init_cat_panel(a: &mut Cursive, cat: &HashMap<&str, Vec<Card>>) {
    let mut list: SelectView<String> = SelectView::new().h_align(HAlign::Left);
    list.set_on_submit(|a, selected: &str| {
        let text = format!("You selected: {}", selected);
        a.add_layer(Dialog::info(text));
    });
    cat.iter().for_each(|(name, cards)| list.add_item(name.to_string(), cards[0].name.to_string()));
    a.add_fullscreen_layer(list);
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
    // BEGIN DANGER: can lead to crashes, should resolve Ok in future
    let file_names = list_files().unwrap();
    // END DANGER
    let card_file_paths = filter_files_by_ext(&file_names, ".cards");
    let card_catalog: HashMap<&str, Vec<Card>> = card_file_paths
        .iter()
        .map(|path| read_json_to_cards(&path))
        .collect();

    let mut app = cursive::default();
    views_init_cat_panel(&mut app, &card_catalog);

    app.load_toml(PATH_COLOR_SCHEME).unwrap();

    app_init_keys(&mut app);

    app.run();
}
