use vizia::prelude::*;

macro_rules! multi_error {
    ($name:ident($($manual:ident),*); $($err:ident = $obj:ty);*) => {
        #[derive(Debug)]
        pub enum $name {
            $($err($obj),)*
            $($manual),*
        }
        
        impl std::fmt::Display for Error { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Debug::fmt(self, f) } }
        impl std::error::Error for Error {}
    
        $(impl From<$obj> for $name { fn from(value: $obj) -> Self { Self::$err(value) } })*
        // $(impl From<Arc<$obj>> for $name { fn from(value: $obj) -> Arc<Self> { Arc::new(Self::$err(value)) } })*
        
    }
}

multi_error! { Error();
    IoError = std::io::Error;
    SpannedError = ron::error::SpannedError;
    ProxyEmitError = ProxyEmitError
}