pub struct HTMLHandlers;

impl HTMLHandlers {

    pub fn is_xss_payload(&self, s: &str) -> bool {
        let s = s.to_lowercase();
        let xss_keywords = [
            "<script", "<iframe", "<img", "<svg", "<link", "<meta", "<object", "<embed",
            "<bgsound", "<video", "<audio", "<marquee", "<form", "<style", "<body",
            "onerror=", "onload=", "onstart=", "onmouseover=", "onclick=", "javascript:",
            "data:text/html", "srcdoc=", "src=", "href=", "url(", "document.cookie",
            "window.location", "window.open", "window.alert", "window.confirm",
            "window.prompt", "eval(", "setTimeout(", "setInterval(", "XMLHttpRequest",
            "ActiveXObject", "iframe src=", "script src=", "object data=", "embed src=",
        ];

        xss_keywords.iter().any(|kw| s.contains(kw))
    }

}
