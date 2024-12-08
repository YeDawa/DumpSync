pub struct Global;

impl Global {

    pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
    pub const APP_AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");
    pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
    pub const APP_LICENSE: &'static str = env!("CARGO_PKG_LICENSE");
    pub const APP_HOMEPAGE: &'static str = env!("CARGO_PKG_HOMEPAGE");

    pub const APP_CONFIGS: &'static str = "https://raw.githubusercontent.com/Kremilly/DumpSync/refs/heads/main/dumpsync.yml";
    pub const XSS_DETECT_REGEX: &'static str = "https://gist.githubusercontent.com/Kremilly/caa28b398966390730a4311337026a0f/raw/15efbaf184d7e3ee3c65cbf79de1f9f1f5425a27/xss-patterns.dsp";

    pub fn app_config() -> String {
        format!("{}.yml", Self::APP_NAME)
    }

}
