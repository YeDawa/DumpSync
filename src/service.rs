use crate::{
    ui::ui_base::UI,
    helpers::env::Env,
    init::DumpSyncInit,

    cloud::{
        push::Push,
        pull::Pull,
        login::Login,
    },
};

pub struct DumpSyncService;

impl DumpSyncService {

    pub fn login(&self) {
        Env::new();
        UI::header();
        UI::section_header("Login to DumpSync", "info");

        let login = Login::new();
        login.print();
        login.save_var();
    }

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

    pub async fn push(&self, path: &str) {
        Env::new();
        UI::header();
        UI::section_header("Pushing dump to server", "info");

        let interval = Env::get_var_u64("DS_DUMP_INTERVAL");
        let (dbname, _, _, _, _) = DumpSyncInit.load_db_config();
        
        Push::new(
            &path,
            &dbname,
            interval,
        ).push().await.expect("Failed to upload SQL file");
    }

}
