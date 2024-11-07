#[rustfmt::skip]
const BANNER: &str = r#"
 _         _           _        _      _   _   
| |__ _  _| |_ ___ ___| |___ _ (_)__ _| |_| |_ 
| '_ \ || |  _/ -_)___| / / ' \| / _` | ' \  _|
|_.__/\_, |\__\___|   |_\_\_||_|_\__, |_||_\__|
      |__/                       |___/         
"#;

pub struct About;
impl About {
    pub const NAME: &'static str = "ByteKnight";
    pub const VERSION: &'static str = "0.1.0";
    pub const SHORT_DESCRIPTION: &'static str = "ByteKnight is a UCI compliant chess engine.";
    pub const AUTHORS: &'static str = "Paul T. (DeveloperPaul123)";
    pub const BANNER: &'static str = BANNER;
}
