pub struct Global;

impl Global {

    pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
    pub const APP_AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");
    pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
    pub const APP_LICENSE: &'static str = env!("CARGO_PKG_LICENSE");
    pub const APP_HOMEPAGE: &'static str = env!("CARGO_PKG_HOMEPAGE");
    
    pub const APP_ICON: &'static str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEYAAABGCAYAAABxLuKEAAAACXBIWXMAAC4jAAAuIwF4pT92AAADSklEQVR4nO2aP07jQBSHf95si0QQQgihIBJRUAfJgtocAY4ARwhHIFfIEcgRNjXVuqFBCGEkhBBCKC5ow5stHENY/Gb8Z5zYu++Tpkk8+jlfZuyZZztKKQjf+bHoE6gqIoZBxDCIGAYRwyBiGEQMg4hhEDEMIoZBxDCIGAYRwyBiGEQMg4hhSCPGA6AstTGAXwB6Gc+zCeB82jdv7gWAk7SBjqmCp5TypidkGx/AoeM4oSG/O81vWsodOo5zbDrIKOb9/b0sMQDgNxqNPU12G8Bv2JMS0280Gme6A4xTSSlVZutOJhN2eCulekqpZgm5vclk0tb97p8mMURkOqQoXU22V2KuB2DAfVkFMew/R0Taf7Ug2ulZBTGVzDaKSfncaQBgmPB5E9FtMhcpsgMAp7ZzAXsjJlhaWholffH29pb1nLJkl5ILzGEqFem/qL6AvalUSv+CfbXrFEQLTJa6j5jmeDxurqysfFs9Ly8v93MHo/5iugDGr6+vQ0QjYLi6uhrkDpyh7mJijqbt/OXlJQAwitva2pp2L8ZRdTEhsu+T2oh20ScA8Pz8/CFpfX1de12ZpepiRohGQhG8acPT01MA4GxjYyNpzfUF4yaSiFK1Iv01fftEFKY9hxStTUQXj4+P57UWs7m56RPRIREFFuWAiHoPDw/aDaq1skOR/jparZbfarU6SqlTpdRQKRVaKj1oxVT6GjPL1tbWANMywf39fXzd8KApWxjQ9quNmFm2t7fjOw3u7u7a+BR0BEvVvlqKmaXT6QT4LDid3t7exoKyFty/UHsxf7Ozs+MD8G9ubv5dMdfX16Yi/HB3dzexPFn67nrBCzxTzTexFlM0F6i+mIX0BezVY7yrq6ukzwvdIVJkH5WRC9gbMR/7EZukLDvkXcdokacEDFUQw5YCiCiA5rlTWbnAHGq+KWDvLEqpAaK3HOaaCyx+xAxc19XdcgeIVrG2ryPaXMBi2SFH67uum/Sw7APXdUMiOiYif565QLqpFMIw7DIQIprbo4ODg1Rlxv39/QDA3uXlZTxy8oyezLnG92P+V+QdPAYRwyBiGEQMg4hhEDEMIoZBxDCIGAYRwyBiGEQMg4hhEDEMIoZBxDCIGAYRw/AHZ4ENamm/WDkAAAAASUVORK5CYII=";

    pub const FORMATS_SUPPORTED: &'static [&'static str] = &["sql", "txt", "xml", "csv", "json", "html"];
    
    pub const DS_API_ENV: &'static str = "DS_API_KEY";
    
    pub fn app_config() -> String {
        format!("{}.yml", Self::APP_NAME)
    }
    
}
