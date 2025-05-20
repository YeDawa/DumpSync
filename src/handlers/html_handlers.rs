pub struct HTMLHandlers;

impl HTMLHandlers {

    pub fn html_escape(&self, input: &str) -> String {
        input
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#39;")
            .replace('`', "&#96;")
            .replace(' ', "&nbsp;")
            .replace('\n', "<br>")
            .replace('\r', "")
            .replace('\t', "&emsp;")
            .replace('(', "&#40;")
            .replace(')', "&#41;")
            .replace('{', "&#123;")
            .replace('}', "&#125;")
            .replace("'", "&#39;")
    }

    pub fn escape_single_quotes(&self, input: &str) -> String {
        input.replace('\'', "''")
    }

}
