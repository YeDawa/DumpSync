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

        let backup_path = Env::get_var("DS_DUMP_PATH");
        let (dbname, host, user, password, port) = DumpSyncInit.load_db_config();

        UI::section_header("Importing dump to server", "info");

        Pull::new(
            &host,
            port,
            &user,
            &password,
            &dbname,
            &backup_path,
            &backup,
        ).import_sql_from_url().await.expect("Failed to download SQL file");
    }

}
