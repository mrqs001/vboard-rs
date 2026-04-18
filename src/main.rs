mod app;
mod config;
mod input;
mod layouts;

fn main() -> anyhow::Result<()> {
    configure_gdk_backend();

    gtk::init()?;
    let app = app::App::new();
    app.run();
    Ok(())
}

fn configure_gdk_backend() {
    // Dragging and saved window coordinates are most reliable with the X11 backend,
    // but explicit user overrides should always win.
    if std::env::var_os("GDK_BACKEND").is_some() {
        return;
    }

    let backend = std::env::var("VBOARD_GDK_BACKEND").unwrap_or_else(|_| "x11".to_string());
    std::env::set_var("GDK_BACKEND", backend);
}
