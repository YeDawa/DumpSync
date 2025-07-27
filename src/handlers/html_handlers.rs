pub struct HTMLHandlers;

impl HTMLHandlers {

    pub fn html_escape(&self, input: &str) -> String {
        input
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#39;")
    }

    pub fn escape_for_sql(&self, input: &str) -> String {
        input
            .replace('\\', "\\\\")
            .replace('\'', "''")
            .replace('\"', "\\\"")
    }

}
