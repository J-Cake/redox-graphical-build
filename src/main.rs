use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use native_dialog::MessageDialog;
use vizia::prelude::*;

mod config;
mod error;

pub use crate::error::Error;

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
        
        Binding::new(cx, AppState::config, |cx, item| {
            if let Some(item) = item.get(cx) {
                println!("{:?}", item);
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
