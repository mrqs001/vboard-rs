use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};
use std::rc::Rc;
use std::time::Duration;

use gtk::gdk;
use gtk::glib;
use gtk::prelude::*;
use gtk::{
    Box as GtkBox, Button, EventBox, Grid, Label, Orientation, PolicyType, ScrolledWindow, Window,
    WindowPosition, WindowType,
};

use crate::config::AppConfig;
use crate::input::{EmitAction, InputBackend};
use crate::layouts::{layout_by_code, KeyBackend, KeyDef, Layout, ALL_LAYOUTS};

const COLOR_PRESETS: &[(&str, &str, &str)] = &[
    ("Pure White", "255,255,255", "solid"),
    ("Pure Black", "0,0,0", "solid"),
    ("Dark Gray", "64,64,64", "solid"),
    ("Medium Gray", "128,128,128", "solid"),
    ("Light Gray", "192,192,192", "solid"),
    ("Red", "255,0,0", "solid"),
    ("Green", "0,255,0", "solid"),
    ("Blue", "0,0,255", "solid"),
    ("Yellow", "255,255,0", "solid"),
    ("Cyan", "0,255,255", "solid"),
    ("Magenta", "255,0,255", "solid"),
    ("Orange", "255,165,0", "solid"),
    ("Purple", "128,0,128", "solid"),
    ("Pink", "255,192,203", "solid"),
    ("Brown", "165,42,42", "solid"),
    (
        "Sky Gradient",
        "linear-gradient(45deg, rgb(135,206,235), rgb(25,25,112))",
        "gradient",
    ),
    (
        "Ocean Gradient",
        "linear-gradient(135deg, rgb(100,149,237), rgb(15,15,60))",
        "gradient",
    ),
    (
        "Sunset Gradient",
        "linear-gradient(45deg, rgb(255,182,193), rgb(220,20,60))",
        "gradient",
    ),
    (
        "Forest Gradient",
        "linear-gradient(45deg, rgb(144,238,144), rgb(0,100,0))",
        "gradient",
    ),
    (
        "Rainbow Gradient",
        "linear-gradient(90deg, rgb(255,0,0), rgb(255,165,0), rgb(255,255,0), rgb(0,255,0), rgb(0,0,255), rgb(128,0,128))",
        "gradient",
    ),
];

const BASIC_COLORS: &[(&str, &str)] = &[
    ("Red", "255,0,0"),
    ("Orange", "255,165,0"),
    ("Yellow", "255,255,0"),
    ("Green", "0,255,0"),
    ("Cyan", "0,255,255"),
    ("Blue", "0,0,255"),
    ("Purple", "128,0,128"),
    ("Pink", "255,192,203"),
    ("White", "255,255,255"),
    ("Light Gray", "192,192,192"),
    ("Gray", "128,128,128"),
    ("Dark Gray", "64,64,64"),
    ("Black", "0,0,0"),
    ("Brown", "165,42,42"),
    ("Maroon", "128,0,0"),
    ("Navy", "0,0,128"),
];

const DIRECTIONS: &[(&str, &str)] = &[
    ("NE", "45deg"),
    ("E", "90deg"),
    ("SE", "135deg"),
    ("S", "180deg"),
    ("SW", "225deg"),
    ("W", "270deg"),
    ("NW", "315deg"),
    ("N", "0deg"),
];

pub struct App {
    config: RefCell<AppConfig>,
    input: RefCell<InputBackend>,
    modifiers: RefCell<BTreeSet<String>>,
    key_defs: RefCell<HashMap<String, KeyDef>>,
    key_buttons: RefCell<HashMap<String, Button>>,
    current_grid: RefCell<Option<Grid>>,
    accessory_buttons: RefCell<Vec<Button>>,
    popup: RefCell<Option<Window>>,
    gradient_colors: RefCell<Vec<(String, String)>>,
    gradient_direction: RefCell<String>,
    drag_state: RefCell<Option<(f64, f64, i32, i32)>>,
    resize_state: RefCell<Option<(f64, f64, i32, i32)>>,
    window: Window,
    top_row: GtkBox,
    drag_strip: EventBox,
    resize_grip: EventBox,
    root: GtkBox,
    toggle_btn: Button,
    opacity_up_btn: Button,
    opacity_down_btn: Button,
    opacity_btn: Button,
    font_up_btn: Button,
    font_down_btn: Button,
    bold_btn: Button,
    position_btn: Button,
    color_btn: Button,
    gradient_btn: Button,
    layout_btn: Button,
    close_btn: Button,
}

impl App {
    pub fn new() -> Rc<Self> {
        let config = AppConfig::load();
        let layout = layout_by_code(&config.keyboard_layout);

        let window = Window::new(WindowType::Popup);
        window.set_title("Virtual Keyboard");
        window.set_resizable(true);
        window.set_keep_above(true);
        window.set_modal(false);
        window.set_decorated(false);
        window.set_focus_on_map(false);
        window.set_can_focus(false);
        window.set_accept_focus(false);
        window.set_skip_taskbar_hint(true);
        window.set_skip_pager_hint(true);
        window.set_type_hint(gdk::WindowTypeHint::PopupMenu);
        window.set_widget_name("toplevel");

        let root = GtkBox::new(Orientation::Vertical, 0);
        window.add(&root);

        let top_bar = EventBox::new();
        top_bar.set_widget_name("top-bar");
        let top_row = GtkBox::new(Orientation::Horizontal, 6);
        top_row.set_margin_start(8);
        top_row.set_margin_end(8);
        top_row.set_margin_top(6);
        top_row.set_margin_bottom(6);
        let drag_strip = EventBox::new();
        drag_strip.set_widget_name("drag-strip");
        drag_strip.set_hexpand(true);
        let resize_grip = EventBox::new();
        resize_grip.set_widget_name("resize-grip");
        let resize_label = Label::new(Some("///"));
        resize_label.set_widget_name("resize-grip-label");
        resize_grip.add(&resize_label);
        top_bar.add(&top_row);
        root.pack_start(&top_bar, false, false, 0);

        let toggle_btn = Self::make_button("Menu");
        let opacity_up_btn = Self::make_button("+");
        let opacity_down_btn = Self::make_button("-");
        let opacity_btn = Self::make_button(&format!("{:.2}", config.opacity));
        let font_up_btn = Self::make_button("A+");
        let font_down_btn = Self::make_button("A-");
        let bold_btn = Self::make_button(if config.bold { "B*" } else { "B" });
        let position_btn = Self::make_button(if config.remember_position {
            "Pos*"
        } else {
            "Pos"
        });
        let color_btn = Self::make_button("Colors");
        let gradient_btn = Self::make_button("Gradient");
        let layout_btn = Self::make_button("Layout");
        let close_btn = Self::make_button("X");

        let app = Rc::new(Self {
            input: RefCell::new(InputBackend::new(layout.xkb_layout)),
            config: RefCell::new(config),
            modifiers: RefCell::new(BTreeSet::new()),
            key_defs: RefCell::new(HashMap::new()),
            key_buttons: RefCell::new(HashMap::new()),
            current_grid: RefCell::new(None),
            accessory_buttons: RefCell::new(Vec::new()),
            popup: RefCell::new(None),
            gradient_colors: RefCell::new(Vec::new()),
            gradient_direction: RefCell::new("45deg".to_string()),
            drag_state: RefCell::new(None),
            resize_state: RefCell::new(None),
            window,
            top_row,
            drag_strip,
            resize_grip,
            root,
            toggle_btn,
            opacity_up_btn,
            opacity_down_btn,
            opacity_btn,
            font_up_btn,
            font_down_btn,
            bold_btn,
            position_btn,
            color_btn,
            gradient_btn,
            layout_btn,
            close_btn,
        });

        app.build_header();
        app.build_footer();
        app.connect_window_signals();
        app.rebuild_keyboard();
        app.apply_css();
        app.restore_geometry();
        app.window.show_all();
        app.change_visibility();
        app
    }

    pub fn run(self: Rc<Self>) {
        gtk::main();
    }

    fn make_button(label: &str) -> Button {
        let button = Button::with_label(label);
        button.set_widget_name("headbar-button");
        button.set_can_focus(false);
        button.set_focus_on_click(false);
        button.set_receives_default(false);
        button
    }

    fn build_header(self: &Rc<Self>) {
        self.top_row.pack_start(&self.toggle_btn, false, false, 0);
        self.top_row
            .pack_start(&self.opacity_up_btn, false, false, 0);
        self.top_row
            .pack_start(&self.opacity_down_btn, false, false, 0);
        self.top_row.pack_start(&self.opacity_btn, false, false, 0);
        self.top_row.pack_start(&self.font_up_btn, false, false, 0);
        self.top_row
            .pack_start(&self.font_down_btn, false, false, 0);
        self.top_row.pack_start(&self.bold_btn, false, false, 0);
        self.top_row.pack_start(&self.position_btn, false, false, 0);
        self.top_row.pack_start(&self.color_btn, false, false, 0);
        self.top_row.pack_start(&self.gradient_btn, false, false, 0);
        self.top_row.pack_start(&self.layout_btn, false, false, 0);
        self.top_row.pack_start(&self.drag_strip, true, true, 0);
        self.top_row.pack_end(&self.close_btn, false, false, 0);

        self.toggle_btn.set_widget_name("menu-button");
        self.close_btn.set_widget_name("close-button");
        self.drag_strip.set_size_request(80, 34);

        self.accessory_buttons.borrow_mut().extend([
            self.opacity_up_btn.clone(),
            self.opacity_down_btn.clone(),
            self.opacity_btn.clone(),
            self.font_up_btn.clone(),
            self.font_down_btn.clone(),
            self.bold_btn.clone(),
            self.position_btn.clone(),
            self.color_btn.clone(),
            self.gradient_btn.clone(),
            self.layout_btn.clone(),
        ]);

        {
            let app = Rc::clone(self);
            self.toggle_btn
                .connect_clicked(move |_| app.change_visibility());
        }
        {
            let app = Rc::clone(self);
            self.opacity_up_btn
                .connect_clicked(move |_| app.change_opacity(true));
        }
        {
            let app = Rc::clone(self);
            self.opacity_down_btn
                .connect_clicked(move |_| app.change_opacity(false));
        }
        {
            let app = Rc::clone(self);
            self.font_up_btn
                .connect_clicked(move |_| app.change_font_size(true));
        }
        {
            let app = Rc::clone(self);
            self.font_down_btn
                .connect_clicked(move |_| app.change_font_size(false));
        }
        {
            let app = Rc::clone(self);
            self.bold_btn.connect_clicked(move |_| app.toggle_bold());
        }
        {
            let app = Rc::clone(self);
            self.position_btn
                .connect_clicked(move |_| app.toggle_position_memory());
        }
        {
            let app = Rc::clone(self);
            self.color_btn
                .connect_clicked(move |_| app.show_color_menu());
        }
        {
            let app = Rc::clone(self);
            self.gradient_btn
                .connect_clicked(move |_| app.show_gradient_builder());
        }
        {
            let app = Rc::clone(self);
            self.layout_btn
                .connect_clicked(move |_| app.show_layout_menu());
        }
        {
            let app = Rc::clone(self);
            self.close_btn.connect_clicked(move |_| {
                app.save_config();
                gtk::main_quit();
            });
        }
        self.drag_strip
            .add_events(gdk::EventMask::BUTTON_PRESS_MASK);
        {
            let app = Rc::clone(self);
            self.drag_strip.connect_button_press_event(move |_, event| {
                if event.button() == 1 {
                    let (x, y) = app.window.position();
                    app.drag_state
                        .replace(Some((event.root().0, event.root().1, x, y)));
                }
                glib::Propagation::Stop
            });
        }
    }

    fn build_footer(self: &Rc<Self>) {
        let footer = GtkBox::new(Orientation::Horizontal, 0);
        footer.set_margin_start(3);
        footer.set_margin_end(3);
        footer.set_margin_bottom(3);
        footer.pack_end(&self.resize_grip, false, false, 0);
        self.root.pack_end(&footer, false, false, 0);

        self.resize_grip.add_events(
            gdk::EventMask::BUTTON_PRESS_MASK
                | gdk::EventMask::BUTTON_RELEASE_MASK
                | gdk::EventMask::POINTER_MOTION_MASK,
        );
        {
            let app = Rc::clone(self);
            self.resize_grip
                .connect_button_press_event(move |_, event| {
                    if event.button() == 1 {
                        let (width, height) = app.window.size();
                        app.resize_state.replace(Some((
                            event.root().0,
                            event.root().1,
                            width,
                            height,
                        )));
                    }
                    glib::Propagation::Stop
                });
        }
    }

    fn connect_window_signals(self: &Rc<Self>) {
        {
            let app = Rc::clone(self);
            self.window.connect_delete_event(move |_, _| {
                app.save_config();
                gtk::main_quit();
                glib::Propagation::Proceed
            });
        }
        {
            let app = Rc::clone(self);
            self.window.connect_configure_event(move |_, _| {
                app.capture_geometry();
                false
            });
        }
        self.window
            .add_events(gdk::EventMask::BUTTON_RELEASE_MASK | gdk::EventMask::POINTER_MOTION_MASK);
        {
            let app = Rc::clone(self);
            self.window.connect_motion_notify_event(move |_, event| {
                let mut handled = false;
                if let Some((start_root_x, start_root_y, start_x, start_y)) =
                    *app.drag_state.borrow()
                {
                    let dx = event.root().0 - start_root_x;
                    let dy = event.root().1 - start_root_y;
                    app.window.move_(start_x + dx as i32, start_y + dy as i32);
                    handled = true;
                }
                if let Some((start_root_x, start_root_y, start_width, start_height)) =
                    *app.resize_state.borrow()
                {
                    let dx = event.root().0 - start_root_x;
                    let dy = event.root().1 - start_root_y;
                    let target_width = (start_width + dx as i32).max(320);
                    let target_height = (start_height + dy as i32).max(180);

                    {
                        let mut config = app.config.borrow_mut();
                        config.width = target_width;
                        config.height = target_height;
                    }

                    app.window.set_default_size(target_width, target_height);
                    app.window.resize(target_width, target_height);
                    handled = true;
                }
                if handled {
                    glib::Propagation::Stop
                } else {
                    glib::Propagation::Proceed
                }
            });
        }
        {
            let app = Rc::clone(self);
            self.window.connect_button_release_event(move |_, _| {
                app.drag_state.take();
                app.resize_state.take();
                glib::Propagation::Proceed
            });
        }
    }

    fn restore_geometry(self: &Rc<Self>) {
        let config = self.config.borrow();
        if config.width > 0 && config.height > 0 {
            self.window.set_default_size(config.width, config.height);
        }
        if config.remember_position && config.pos_x >= 0 && config.pos_y >= 0 {
            let window = self.window.clone();
            let x = config.pos_x;
            let y = config.pos_y;
            glib::timeout_add_local_once(Duration::from_millis(50), move || {
                window.move_(x, y);
            });
        }
    }

    fn capture_geometry(&self) {
        let (width, height) = self.window.size();
        let (x, y) = self.window.position();
        let mut config = self.config.borrow_mut();
        config.width = width;
        config.height = height;
        if config.remember_position {
            config.pos_x = x;
            config.pos_y = y;
        }
    }

    fn save_config(&self) {
        if let Err(err) = self.config.borrow().save() {
            eprintln!("failed to save config: {err}");
        }
    }

    fn current_layout(&self) -> Layout {
        layout_by_code(&self.config.borrow().keyboard_layout)
    }

    fn rebuild_keyboard(self: &Rc<Self>) {
        if let Some(grid) = self.current_grid.borrow_mut().take() {
            self.root.remove(&grid);
        }

        self.key_defs.borrow_mut().clear();
        self.key_buttons.borrow_mut().clear();

        let layout = self.current_layout();
        let grid = Grid::new();
        grid.set_row_homogeneous(true);
        grid.set_column_homogeneous(true);
        grid.set_margin_start(3);
        grid.set_margin_end(3);
        grid.set_widget_name("grid");

        for (row_index, row) in layout.rows.iter().enumerate() {
            let mut col = 0;
            for key in row.iter().copied() {
                let label = key.label(self.shift_active());
                let button = Button::with_label(label);
                button.set_can_focus(false);
                button.set_focus_on_click(false);
                button.set_receives_default(false);
                button.set_hexpand(true);
                button.set_vexpand(true);
                let key_id = key.id.to_string();
                self.key_defs.borrow_mut().insert(key_id.clone(), key);
                self.key_buttons
                    .borrow_mut()
                    .insert(key_id.clone(), button.clone());

                let app = Rc::clone(self);
                button.connect_clicked(move |_| app.on_key_clicked(&key_id));
                grid.attach(&button, col, row_index as i32, key.width, 1);
                col += key.width;
            }
        }

        self.root.pack_start(&grid, true, true, 0);
        grid.show_all();
        self.current_grid.replace(Some(grid));
        self.apply_css();
    }

    fn shift_active(&self) -> bool {
        let modifiers = self.modifiers.borrow();
        modifiers.contains("shift_l") || modifiers.contains("shift_r")
    }

    fn update_key_labels(&self) {
        let shifted = self.shift_active();
        let key_defs = self.key_defs.borrow();
        let key_buttons = self.key_buttons.borrow();
        for (id, key) in key_defs.iter() {
            if let Some(button) = key_buttons.get(id) {
                button.set_label(key.label(shifted));
            }
        }
    }

    fn on_key_clicked(self: &Rc<Self>, key_id: &str) {
        let Some(key) = self.key_defs.borrow().get(key_id).copied() else {
            return;
        };

        if let KeyBackend::Modifier(_) = key.backend {
            self.toggle_modifier(key_id);
            return;
        }

        let shifted = self.shift_active();
        let modifiers =
            self.active_backend_modifiers(matches!(key.backend, KeyBackend::Special(_)));
        let action = match key.backend {
            KeyBackend::Text => EmitAction::Text(key.text_output(shifted).to_string()),
            KeyBackend::Special(name) => EmitAction::Special(name),
            KeyBackend::Modifier(_) => return,
        };

        let layout = self.current_layout();
        self.input.borrow_mut().set_layout(layout.xkb_layout);

        if let Err(err) = self.input.borrow_mut().emit(action, &modifiers) {
            self.show_error(&err);
        }

        self.reset_modifiers();
    }

    fn toggle_modifier(&self, key_id: &str) {
        let mut modifiers = self.modifiers.borrow_mut();
        if modifiers.contains(key_id) {
            modifiers.remove(key_id);
        } else {
            modifiers.insert(key_id.to_string());
        }

        if modifiers.contains("shift_l") && modifiers.contains("shift_r") {
            modifiers.remove("shift_l");
            modifiers.remove("shift_r");
        }

        drop(modifiers);
        self.update_key_labels();
    }

    fn reset_modifiers(&self) {
        self.modifiers.borrow_mut().clear();
        self.update_key_labels();
    }

    fn active_backend_modifiers(&self, include_shift: bool) -> Vec<String> {
        let key_defs = self.key_defs.borrow();
        let modifiers = self.modifiers.borrow();
        let mut result = BTreeSet::new();

        for id in modifiers.iter() {
            let Some(key) = key_defs.get(id) else {
                continue;
            };
            let KeyBackend::Modifier(name) = key.backend else {
                continue;
            };
            if !include_shift && name == "shift" {
                continue;
            }
            result.insert(name.to_string());
        }

        result.into_iter().collect()
    }

    fn change_visibility(&self) {
        for button in self.accessory_buttons.borrow().iter() {
            button.set_visible(!button.is_visible());
        }
    }

    fn change_opacity(&self, increase: bool) {
        let mut config = self.config.borrow_mut();
        let step = 0.01_f64;
        if increase {
            config.opacity = (config.opacity + step).min(1.0);
        } else {
            config.opacity = (config.opacity - step).max(0.0);
        }
        self.opacity_btn
            .set_label(&format!("{:.2}", config.opacity));
        drop(config);
        self.apply_css();
    }

    fn change_font_size(&self, increase: bool) {
        let mut config = self.config.borrow_mut();
        if increase {
            config.font_size = (config.font_size + 1).min(48);
        } else {
            config.font_size = (config.font_size - 1).max(8);
        }
        drop(config);
        self.apply_css();
    }

    fn toggle_bold(&self) {
        let mut config = self.config.borrow_mut();
        config.bold = !config.bold;
        self.bold_btn
            .set_label(if config.bold { "B*" } else { "B" });
        drop(config);
        self.apply_css();
    }

    fn toggle_position_memory(&self) {
        let mut config = self.config.borrow_mut();
        config.remember_position = !config.remember_position;
        self.position_btn.set_label(if config.remember_position {
            "Pos*"
        } else {
            "Pos"
        });
        if config.remember_position {
            let (x, y) = self.window.position();
            config.pos_x = x;
            config.pos_y = y;
        }
    }

    fn apply_css(&self) {
        let config = self.config.borrow();
        let provider = gtk::CssProvider::new();
        let font_weight = if config.bold { "bold" } else { "normal" };
        let bg_css = if config.bg_type == "gradient" {
            format!("background: {};", config.bg_color)
        } else {
            format!(
                "background-color: rgba({}, {:.2});",
                config.bg_color, config.opacity
            )
        };

        let css = format!(
            "
            #top-bar {{
                {bg_css}
                border: 0px;
                box-shadow: none;
            }}
            #top-bar button {{
                min-width: 52px;
                min-height: 36px;
                padding: 2px 8px;
                border: 0px;
                margin: 0px;
            }}
            #top-bar button label {{
                color: {text_color};
                font-size: 14px;
                font-weight: {font_weight};
            }}
            #drag-strip {{
                min-height: 36px;
            }}
            #resize-grip {{
                min-width: 26px;
                min-height: 22px;
                padding: 0px;
                margin: 0px;
                border: 1px solid rgba(255, 255, 255, 0.20);
                border-radius: 4px;
                background-image: none;
                background-color: rgba(255, 255, 255, 0.06);
            }}
            #resize-grip-label {{
                color: {text_color};
                font-size: 10px;
                font-weight: bold;
            }}
            #menu-button label {{
                font-weight: bold;
            }}
            #close-button label {{
                font-weight: bold;
            }}
            #headbar-button, #drag-strip {{
                background-image: none;
            }}
            #toplevel {{
                {bg_css}
            }}
            #grid button label {{
                color: {text_color};
                font-size: {font_size}px;
                font-weight: {font_weight};
            }}
            #grid button {{
                border: 1px solid rgba(255, 255, 255, 0.3);
                background-image: none;
                margin: 1px;
            }}
            #grid button:hover {{
                border: 1px solid #00CACB;
            }}
            button {{
                background-color: transparent;
                color: {text_color};
            }}
            ",
            bg_css = bg_css,
            text_color = config.text_color,
            font_size = config.font_size,
            font_weight = font_weight,
        );

        if let Err(err) = provider.load_from_data(css.as_bytes()) {
            eprintln!("css error: {err}");
            return;
        }

        if let Some(screen) = gdk::Screen::default() {
            gtk::StyleContext::add_provider_for_screen(
                &screen,
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_USER,
            );
        }
    }

    fn show_error(&self, message: &str) {
        eprintln!("{message}");
    }

    fn close_popup(&self) {
        if let Some(window) = self.popup.borrow_mut().take() {
            unsafe {
                window.destroy();
            }
        }
    }

    fn make_popup(self: &Rc<Self>, title: &str, width: i32, height: i32) -> Window {
        self.close_popup();

        let popup = Window::new(WindowType::Toplevel);
        popup.set_title(title);
        popup.set_transient_for(Some(&self.window));
        popup.set_destroy_with_parent(true);
        popup.set_modal(true);
        popup.set_position(WindowPosition::CenterOnParent);
        popup.set_default_size(width, height);

        let app = Rc::clone(self);
        popup.connect_delete_event(move |_, _| {
            app.popup.borrow_mut().take();
            glib::Propagation::Proceed
        });

        self.popup.replace(Some(popup.clone()));
        popup
    }

    fn show_color_menu(self: &Rc<Self>) {
        let popup = self.make_popup("Colors", 320, 420);
        let scrolled = ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
        scrolled.set_policy(PolicyType::Never, PolicyType::Automatic);
        let grid = Grid::new();
        grid.set_row_spacing(4);
        grid.set_margin_start(10);
        grid.set_margin_end(10);
        grid.set_margin_top(10);
        grid.set_margin_bottom(10);

        for (row, (label, color, bg_type)) in COLOR_PRESETS.iter().enumerate() {
            let button = Button::with_label(label);
            button.set_can_focus(false);
            button.set_focus_on_click(false);
            button.set_receives_default(false);
            let color = (*color).to_string();
            let bg_type = (*bg_type).to_string();
            let app = Rc::clone(self);
            button.connect_clicked(move |_| {
                let mut config = app.config.borrow_mut();
                config.bg_color = color.clone();
                config.bg_type = bg_type.clone();
                config.update_text_color();
                drop(config);
                app.apply_css();
                app.close_popup();
            });
            grid.attach(&button, 0, row as i32, 1, 1);
        }

        scrolled.add(&grid);
        popup.add(&scrolled);
        popup.show_all();
    }

    fn show_layout_menu(self: &Rc<Self>) {
        let popup = self.make_popup("Layouts", 260, 180);
        let scrolled = ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
        scrolled.set_policy(PolicyType::Never, PolicyType::Automatic);
        let grid = Grid::new();
        grid.set_row_spacing(4);
        grid.set_margin_start(10);
        grid.set_margin_end(10);
        grid.set_margin_top(10);
        grid.set_margin_bottom(10);

        let current = self.config.borrow().keyboard_layout.clone();
        for (row, layout) in ALL_LAYOUTS.iter().enumerate() {
            let label = if layout.code == current {
                format!("* {} ({})", layout.name, layout.code)
            } else {
                format!("{} ({})", layout.name, layout.code)
            };
            let button = Button::with_label(&label);
            button.set_can_focus(false);
            button.set_focus_on_click(false);
            let code = layout.code.to_string();
            let app = Rc::clone(self);
            button.connect_clicked(move |_| {
                app.config.borrow_mut().keyboard_layout = code.clone();
                let layout = layout_by_code(&code);
                app.input.borrow_mut().set_layout(layout.xkb_layout);
                app.rebuild_keyboard();
                app.close_popup();
            });
            grid.attach(&button, 0, row as i32, 1, 1);
        }

        scrolled.add(&grid);
        popup.add(&scrolled);
        popup.show_all();
    }

    fn show_gradient_builder(self: &Rc<Self>) {
        self.gradient_colors.borrow_mut().clear();
        self.gradient_direction.replace("45deg".to_string());

        let popup = self.make_popup("Gradient Builder", 460, 520);
        let outer = GtkBox::new(Orientation::Vertical, 8);
        outer.set_margin_start(12);
        outer.set_margin_end(12);
        outer.set_margin_top(12);
        outer.set_margin_bottom(12);

        let title = Label::new(Some("Build a custom gradient"));
        outer.pack_start(&title, false, false, 0);

        let direction_box = GtkBox::new(Orientation::Horizontal, 4);
        for (label, direction) in DIRECTIONS {
            let button = Button::with_label(label);
            button.set_can_focus(false);
            button.set_focus_on_click(false);
            let direction = (*direction).to_string();
            let app = Rc::clone(self);
            button.connect_clicked(move |_| {
                app.gradient_direction.replace(direction.clone());
            });
            direction_box.pack_start(&button, true, true, 0);
        }
        outer.pack_start(&direction_box, false, false, 0);

        let selected_label = Label::new(Some("Selected colors"));
        outer.pack_start(&selected_label, false, false, 0);

        let selected_colors_box = GtkBox::new(Orientation::Vertical, 4);
        let selected_scroll =
            ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
        selected_scroll.set_policy(PolicyType::Never, PolicyType::Automatic);
        selected_scroll.set_size_request(-1, 120);
        selected_scroll.add(&selected_colors_box);
        outer.pack_start(&selected_scroll, false, false, 0);

        self.clone().refresh_selected_colors(&selected_colors_box);

        let controls = GtkBox::new(Orientation::Horizontal, 6);
        let clear_btn = Button::with_label("Clear");
        let apply_btn = Button::with_label("Apply");
        let cancel_btn = Button::with_label("Cancel");
        for button in [&clear_btn, &apply_btn, &cancel_btn] {
            button.set_can_focus(false);
            button.set_focus_on_click(false);
        }

        {
            let app = Rc::clone(self);
            let selected_colors_box = selected_colors_box.clone();
            clear_btn.connect_clicked(move |_| {
                app.gradient_colors.borrow_mut().clear();
                app.clone().refresh_selected_colors(&selected_colors_box);
            });
        }
        {
            let app = Rc::clone(self);
            apply_btn.connect_clicked(move |_| {
                let colors = app.gradient_colors.borrow();
                if colors.is_empty() {
                    app.close_popup();
                    return;
                }

                let mut config = app.config.borrow_mut();
                if colors.len() == 1 {
                    config.bg_color = colors[0].0.clone();
                    config.bg_type = "solid".to_string();
                } else {
                    let stops = colors
                        .iter()
                        .map(|(color, _)| format!("rgb({color})"))
                        .collect::<Vec<_>>()
                        .join(", ");
                    config.bg_color = format!(
                        "linear-gradient({}, {})",
                        app.gradient_direction.borrow().as_str(),
                        stops
                    );
                    config.bg_type = "gradient".to_string();
                }
                config.update_text_color();
                drop(config);
                app.apply_css();
                app.close_popup();
            });
        }
        {
            let app = Rc::clone(self);
            cancel_btn.connect_clicked(move |_| app.close_popup());
        }

        controls.pack_start(&clear_btn, true, true, 0);
        controls.pack_start(&apply_btn, true, true, 0);
        controls.pack_start(&cancel_btn, true, true, 0);
        outer.pack_start(&controls, false, false, 0);

        let colors_label = Label::new(Some("Pick colors"));
        outer.pack_start(&colors_label, false, false, 0);

        let color_scroll = ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
        color_scroll.set_policy(PolicyType::Never, PolicyType::Automatic);
        color_scroll.set_size_request(-1, 220);
        let color_grid = Grid::new();
        color_grid.set_row_spacing(4);
        color_grid.set_column_spacing(4);

        for (index, (label, color)) in BASIC_COLORS.iter().enumerate() {
            let button = Button::with_label(label);
            button.set_can_focus(false);
            button.set_focus_on_click(false);
            let label = (*label).to_string();
            let color = (*color).to_string();
            let app = Rc::clone(self);
            let selected_colors_box = selected_colors_box.clone();
            button.connect_clicked(move |_| {
                app.gradient_colors
                    .borrow_mut()
                    .push((color.clone(), label.clone()));
                app.clone().refresh_selected_colors(&selected_colors_box);
            });
            color_grid.attach(&button, (index % 4) as i32, (index / 4) as i32, 1, 1);
        }

        color_scroll.add(&color_grid);
        outer.pack_start(&color_scroll, true, true, 0);

        popup.add(&outer);
        popup.show_all();
    }

    fn refresh_selected_colors(self: Rc<Self>, container: &GtkBox) {
        for child in container.children() {
            container.remove(&child);
        }

        let colors = self.gradient_colors.borrow().clone();
        if colors.is_empty() {
            let label = Label::new(Some("No colors selected"));
            container.pack_start(&label, false, false, 0);
        } else {
            for (index, (_, name)) in colors.iter().enumerate() {
                let row = GtkBox::new(Orientation::Horizontal, 4);
                let label = Label::new(Some(&format!("{}. {}", index + 1, name)));
                let remove = Button::with_label("x");
                remove.set_can_focus(false);
                remove.set_focus_on_click(false);
                let app = Rc::clone(&self);
                let refresh_container = container.clone();
                remove.connect_clicked(move |_| {
                    if index < app.gradient_colors.borrow().len() {
                        app.gradient_colors.borrow_mut().remove(index);
                        app.clone().refresh_selected_colors(&refresh_container);
                    }
                });
                row.pack_start(&label, true, true, 0);
                row.pack_start(&remove, false, false, 0);
                container.pack_start(&row, false, false, 0);
            }
        }

        container.show_all();
    }
}
