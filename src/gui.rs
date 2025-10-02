use gtk4::gdk;
use gtk4::glib::Propagation;
use gtk4::pango::EllipsizeMode;
use std::cell::RefCell;
use std::rc::Rc;

use crate::cache::load_cache;
use crate::desktop::DesktopEntry;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use gtk4::CssProvider;
use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box as GtkBox, Entry, ListBox, ListBoxRow, Orientation,
};

#[derive(Clone)]
pub enum SearchResult {
    App(DesktopEntry),
    CalcResult {
        display: String,
        value_to_copy: String,
    },
}

pub fn run_gui() {
    // Create GTK application
    let app = Application::builder()
        .application_id("com.example.bazooka")
        .build();

    // Build UI inside activate handler
    app.connect_activate(|app| {
        let all_apps: Vec<DesktopEntry> = load_cache().unwrap_or_default();
        let results = Rc::new(RefCell::new(Vec::<SearchResult>::new()));

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Bazooka")
            .default_width(580)
            .default_height(625)
            .resizable(false)
            .opacity(1.0)
            .build();

        let provider = CssProvider::new();
        provider.load_from_data(include_str!("style.css"));
        gtk4::style_context_add_provider_for_display(
            &gtk4::prelude::WidgetExt::display(&window),
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_USER,
        );

        window.add_css_class("window");

        // vertical layout
        let vbox = GtkBox::new(Orientation::Vertical, 8);

        // search bar
        let entry = Entry::new();
        entry.add_css_class("search-entry");
        vbox.append(&entry);

        // results
        let list = ListBox::new();
        list.add_css_class("results-list");
        vbox.append(&list);

        window.set_child(Some(&vbox));
        window.show();

        entry.grab_focus();

        // fuzzy search on input
        let list_clone = list.clone();
        let results_for_changed = Rc::clone(&results);
        entry.connect_changed(move |entry| {
            let query = entry.text().to_string();
            let mut current_results: Vec<SearchResult> = Vec::new();

            if let Ok(result) = meval::eval_str(&query)
                && result.is_finite()
            {
                current_results.push(SearchResult::CalcResult {
                    display: format!("{}", result),
                    value_to_copy: result.to_string(),
                })
            }

            if current_results.is_empty() && !query.is_empty() {
                let matcher = SkimMatcherV2::default();
                let filtered: Vec<DesktopEntry> = all_apps
                    .iter()
                    .filter(|app| matcher.fuzzy_match(&app.name, &query).is_some())
                    .take(8)
                    .cloned()
                    .collect();
                current_results = filtered.into_iter().map(SearchResult::App).collect();
            }

            while let Some(child) = list_clone.first_child() {
                list_clone.remove(&child);
            }

            for result in &current_results {
                let row = ListBoxRow::new();
                row.set_activatable(true);
                row.add_css_class("result-row");

                let hbox = GtkBox::new(Orientation::Horizontal, 12);

                match result {
                    SearchResult::App(app) => {
                        let hbox = GtkBox::new(Orientation::Horizontal, 12);
                        hbox.add_css_class("result-row-content");

                        let image = if let Some(primary_icon) = &app.icon {
                            let theme = gtk4::IconTheme::default();
                            if theme.has_icon(primary_icon) {
                                gtk4::Image::from_icon_name(primary_icon)
                            } else {
                                let fallback_icon = app.name.to_lowercase();
                                if theme.has_icon(&fallback_icon) {
                                    gtk4::Image::from_icon_name(&fallback_icon)
                                } else {
                                    gtk4::Image::from_icon_name("application-x-executable-symbolic")
                                }
                            }
                        } else {
                            gtk4::Image::from_icon_name("application-x-executable-symbolic")
                        };
                        image.set_icon_size(gtk4::IconSize::Large);
                        hbox.append(&image);

                        let vbox = GtkBox::new(Orientation::Vertical, 5);
                        let label = gtk4::Label::new(Some(&app.name));
                        label.set_halign(gtk4::Align::Start);
                        vbox.append(&label);

                        if let Some(comment) = &app.comment {
                            let comment_label = gtk4::Label::new(Some(comment));
                            comment_label.set_halign(gtk4::Align::Start);
                            comment_label.add_css_class("result-comment");
                            comment_label.set_ellipsize(EllipsizeMode::End);
                            vbox.append(&comment_label);
                        } else {
                            let comment_label = gtk4::Label::new(Some(" "));
                            comment_label.set_halign(gtk4::Align::Start);
                            comment_label.add_css_class("result-comment");
                            comment_label.set_ellipsize(EllipsizeMode::End);
                            vbox.append(&comment_label);
                        }
                        hbox.append(&vbox);

                        let row = ListBoxRow::new();
                        row.set_child(Some(&hbox));
                        row.add_css_class("result-row");
                        row.set_activatable(true);
                        list_clone.append(&row);
                    }
                    SearchResult::CalcResult { display, .. } => {
                        let image = gtk4::Image::from_icon_name("accessories-calculator-symbolic");
                        image.set_icon_size(gtk4::IconSize::Large);
                        hbox.add_css_class("result-row-content");
                        hbox.append(&image);

                        let vbox = GtkBox::new(Orientation::Vertical, 5);
                        let label = gtk4::Label::new(Some(display));
                        label.set_halign(gtk4::Align::Start);
                        vbox.append(&label);

                        let comment_label =
                            gtk4::Label::new(Some("Press Enter to copy the result."));
                        comment_label.set_halign(gtk4::Align::Start);
                        comment_label.add_css_class("result-comment");
                        vbox.append(&comment_label);

                        hbox.append(&vbox);
                        row.set_child(Some(&hbox));
                        row.add_css_class("result-row");
                        row.set_activatable(true);

                        list_clone.append(&row);
                    }
                }
                // list_clone.append(&row);
            }

            *results_for_changed.borrow_mut() = current_results;
            if let Some(first_row) = list_clone.row_at_index(0) {
                list_clone.select_row(Some(&first_row));
            }
        });

        // Handle row activation (click or Enter)
        let window_clone = window.clone();
        let results_clone_for_activate = Rc::clone(&results);
        list.connect_row_activated(move |_, row| {
            let index = row.index();
            if index >= 0
                && let Some(result) = results_clone_for_activate.borrow().get(index as usize)
            {
                match result {
                    SearchResult::App(app) => {
                        let _ = std::process::Command::new("sh")
                            .arg("-c")
                            .arg(&app.exec)
                            .spawn();
                        window_clone.close();
                    }
                    SearchResult::CalcResult { value_to_copy, .. } => {
                        let display = gdk::Display::default().expect("Could not get display");
                        display.clipboard().set_text(value_to_copy);
                    }
                }
            }
        });

        let key_controller = gtk4::EventControllerKey::new();
        key_controller.set_propagation_phase(gtk4::PropagationPhase::Capture);

        let list_clone = list.clone();
        let window_clone = window.clone();
        key_controller.connect_key_pressed(move |_, keyval, _, modifier| {
            let list = &list_clone;

            let get_state = || -> Option<(i32, u32)> {
                let selected_row = list.selected_row()?;
                let index = selected_row.index();
                let total = list.observe_children().n_items();
                Some((index, total))
            };

            let move_selection = |delta: i32| {
                if let Some((index, total)) = get_state() {
                    let new_index = (index + delta).clamp(0, total as i32 - 1);
                    if let Some(new_row) = list.row_at_index(new_index) {
                        list.select_row(Some(&new_row));
                    }
                }
            };

            let is_ctrl = modifier.contains(gdk::ModifierType::CONTROL_MASK);
            if is_ctrl {
                match keyval {
                    gtk4::gdk::Key::n => move_selection(1),
                    gtk4::gdk::Key::p => move_selection(-1),
                    _ => return Propagation::Proceed,
                }
                return Propagation::Stop;
            }

            match keyval {
                gtk4::gdk::Key::Up => move_selection(-1),
                gtk4::gdk::Key::Down => move_selection(1),
                gtk4::gdk::Key::Escape => {
                    window_clone.close();
                }
                gtk4::gdk::Key::Return | gtk4::gdk::Key::KP_Enter => {
                    if let Some(selected_row) = list.selected_row() {
                        list.emit_by_name::<()>("row-activated", &[&selected_row]);
                    }
                }
                _ => return Propagation::Proceed,
            }

            Propagation::Stop
        });

        entry.add_controller(key_controller);
    });

    // Run the GTK main loop
    app.run();
}
