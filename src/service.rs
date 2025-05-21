use crate::{
    init::DumpSyncInit,

    ui::ui_base::UI,
    helpers::env::Env,
    cloud::pull::Pull,
};

pub struct DumpSyncService;

impl DumpSyncService {

    pub async fn pull(&self, backup: &str) {
        Env::new();
        UI::header();

        let (dbname, host, user, password, port) = DumpSyncInit.load_db_config();
        UI::section_header("Importing dump to server", "info");

        Pull::new(
            &host,
            port,
            &user,
            &password,
            &dbname,
            &backup,
        ).pull().await.expect("Failed to download SQL file");
    }

}
