use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use native_dialog::MessageDialog;
use vizia::prelude::*;

mod config;
mod components;
mod error;
mod builder;

pub use crate::error::Error;
use crate::builder::Builder;

#[derive(Lens)]
struct AppState {
    config: Option<config::BuildConfig>
}

#[derive(Clone)]
pub enum AppEvent {
    LoadConfig(config::BuildConfig),
    Error(Arc<Error>)
}

impl Model for AppState {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
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
            }
        }
    }
}

struct FallbackScreen {}
impl View for FallbackScreen {}

fn load_file(cx: &mut ContextProxy) -> Result<(), Error> {
    if let Ok(res) = native_dialog::FileDialog::new()
        .set_title("Open Configuration")
        .set_location("~/")
        .add_filter("Redox OS Build configuration", &["ron"])
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
        AppState { config: None }.build(cx);
        
        cx.add_stylesheet(r#"
            :root {
                background-color: #f1f1f1;
                font-family: "Segoe UI", Arial, sans-serif;
            }

            button {
                height: 24px;
                border-width: 0px;
                border-radius: 0px;
                outline-width: 0px;
                background-color: #d4d4d4;
                color: #454545;
                transition: background-color 0ms;
                font-size: small;
                col-between: 3px;
                row-between: 2px;
            }
            
            button.primary {
                background-color: #569de8;
                color: #eaeaea;
            }
            
            button, button:hover, button:over, button:active, button:focus-visible {
                border: none;
                outline: none;
            }
            
            button.primary:hover, button.primary:over {
                background-color: #4175ab;                
            }
            
            .field {
                border: 1px solid #8d8d8d;
                height: 1s;
                width: 1s;
                top: 4px;
                left: 4px;
                right: 4px;
                bottom: 4px;
            }
            
            table > .table-header > .table-header-column {
                font-size: small;
                color: #666666;
                background-color: #b4b4b4;
                height: auto;
            }
        "#)
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
    .inner_size((720, 480))
    .run();
}
