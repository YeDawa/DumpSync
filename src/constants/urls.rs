pub enum UrlsNames {
    AppConfigs,
    XssDetectRegex,
    PastebinApiUri,
    CdnBootstrap,
    DumpsyncApi,
    DumpsyncApiKey,
    DownloadFilesUri,
}

pub struct Urls;

impl Urls {

    const APP_CONFIGS: &'static str = "https://raw.githubusercontent.com/YeDawa/DumpSync/refs/heads/main/dumpsync.yml";
    const XSS_DETECT_REGEX: &'static str = "https://raw.githubusercontent.com/YeDawa/DumpSync/refs/heads/main/patterns.txt";
    
    const PASTEBIN_API_URI: &'static str = "https://pastebin.com/api/api_post.php";
    const DUMPSYNC_API: &'static str = "https://service.dumpsync.com/";
    const DUMPSYNC_API_KEY: &'static str = "https://dumpsync.com/dashboard/settings/api-key";

    const CDN_BOOTSTRAP: &'static str = "https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css";
    const DOWNLOAD_FILES_URI: &'static str = "https://raw.githubusercontent.com/YeDawa/DumpSync/main/";

    pub fn as_str(urls_name: UrlsNames) -> &'static str {
        match urls_name {
            UrlsNames::AppConfigs => Urls::APP_CONFIGS,
            UrlsNames::XssDetectRegex => Urls::XSS_DETECT_REGEX,
            UrlsNames::PastebinApiUri => Urls::PASTEBIN_API_URI,
            UrlsNames::DumpsyncApi => Urls::DUMPSYNC_API,
            UrlsNames::DumpsyncApiKey => Urls::DUMPSYNC_API_KEY,
            UrlsNames::DownloadFilesUri => Urls::DOWNLOAD_FILES_URI,
            UrlsNames::CdnBootstrap => Urls::CDN_BOOTSTRAP,
        }
    }
    
}
