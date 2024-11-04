pub struct Global;

impl Global {

    pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
    pub const APP_AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");
    pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
    pub const APP_LICENSE: &'static str = env!("CARGO_PKG_LICENSE");
    pub const APP_HOMEPAGE: &'static str = env!("CARGO_PKG_HOMEPAGE");

    pub const APP_CONFIGS: &'static str = "https://gist.githubusercontent.com/Kremilly/0189fecf4fe7d1b86f35beb8e14f6091/raw/dc34fc70411afbcee76d8afef84739c25d291654/dumpsync.yml";

    pub fn app_config() -> String {
        format!("{}.yml", Self::APP_NAME)
    }

}
