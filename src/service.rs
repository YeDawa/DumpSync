use crate::{
    init::DumpSyncInit,
    core::encrypt::Encrypt,

    ui::ui_base::UI,
    helpers::env::Env,

    cloud::{
        push::Push,
        pull::Pull,
    },
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

    pub async fn push(&self, path: &str) {
        Env::new();
        UI::header();
        UI::section_header("Pushing dump to server", "info");

        let (dbname, _, _, _, _) = DumpSyncInit.load_db_config();
        let encrypt = Encrypt::new(path).calculate_entropy();

        let encrypted = match encrypt {
            Ok(val) => val > 0.5,
            Err(e) => {
                eprintln!("Failed to calculate entropy: {}", e);
                false
            }
        };
        
        Push::new(
            &path,
            &dbname,
            encrypted,
        ).push().await.expect("Failed to upload SQL file");
    }

}
