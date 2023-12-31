use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::sync::Arc;
use native_dialog::MessageDialog;
use vizia::prelude::*;

mod components;
mod error;
mod build;
mod edit;

pub use crate::error::Error;
use crate::build::Builder;

#[derive(Lens)]
struct AppState {
    config: Option<build::BuildConfig>,
    started: Option<Instant>,
}

#[derive(Clone)]
pub enum AppEvent {
    LoadConfig(build::BuildConfig),
    Error(Arc<Error>),
    BuildStarted(bool),
    Refresh
}

impl Model for AppState {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        match event.take() {
            None => return,
            Some(AppEvent::LoadConfig(config)) => self.config = Some(config.clone()),
            Some(AppEvent::Error(err)) => {
                let err = Arc::clone(&err);
                std::thread::spawn(move || MessageDialog::new()
                    .set_title("Error")
                    .set_type(native_dialog::MessageType::Error)
                    .set_text(&err.to_string())
                    .show_alert()
                    .unwrap());
            },
            Some(AppEvent::BuildStarted(started)) => self.started = if started { Some(Instant::now()) } else { None },
            Some(AppEvent::Refresh) => {}
        }
    }
}

struct FallbackScreen {}
impl View for FallbackScreen {}

fn load_file(cx: &mut ContextProxy) -> Result<(), Error> {    
    if let Ok(res) = native_dialog::FileDialog::new()
        .set_title("Open Configuration")
        .set_location("~/")
        .add_filter("Redox OS Build configuration", &["ron", "json", "json5", "toml", "cson"])
        .show_open_single_file() {
            
        let Some(path) = res else { return Ok(()); };
        
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(err) => {
                cx.emit(AppEvent::Error(Arc::new(Error::from(err))))?;
                return Ok(());
            }
        };
        
        let mut buf = String::new();
        if let Err(err) = file.read_to_string(&mut buf) {
            cx.emit(AppEvent::Error(Arc::new(Error::from(err))))?;
        }
        
        let config = match ron::from_str(&buf) {
            Ok(config) => config,
            Err(err) => { 
                cx.emit(AppEvent::Error(Arc::new(Error::from(err))))?;
                return Ok(())
            }
        };
        
        cx.emit(AppEvent::LoadConfig(config))?;
    };
    
    Ok(())    
}

fn main() {
    Application::new(|cx| {
        AppState { 
            config: None, 
            started: None
        }.build(cx);
        
        cx.spawn(|cx| loop {
            std::thread::sleep(Duration::from_millis(1000));
            cx.emit(AppEvent::Refresh).unwrap_or_default();
        });

        cx.add_stylesheet(PathBuf::from("./theme.css"))
            .expect("Failed to load stylesheet");
            
        Binding::new(cx, AppState::config, |cx, item| {
            if let Some(config) = item.get(cx) {
                Builder::new(cx, config);
            } else {
                HStack::new(cx, |cx| {
                    Button::new(cx, |event| event.spawn(|cx| {
                        if let Err(err) = load_file(cx) {
                            cx.emit(AppEvent::Error(Arc::new(err)))
                                .expect("Failed to propagate error");
                        }
                    }), |cx| Label::new(cx, "Open"));
                });
            }
        });
    })
    .title("Redox Builder")
    .inner_size((480, 320))
    .min_inner_size(Some((220, 160)))
    .run();
}
