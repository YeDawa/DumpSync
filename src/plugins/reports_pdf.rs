use printpdf::*;

use std::{
    io::BufWriter,

    fs::{
        self, 
        File
    },
};

use crate::{
    constants::global::Global,
    ui::report_alerts::ReportAlerts,
    handlers::reports_handlers::ReportsHandlers,
};

pub struct ReportsPdfs {
    file: String,
    path: String,
    interval: usize,
    counter: usize,
}

impl ReportsPdfs {

    pub fn new(file: &str, path: &str, interval: usize, counter: usize) -> Self {
        Self {
            file: file.to_string(),
            path: path.to_string(),
            interval,
            counter,
        }
    }

    fn read_yaml_as_text(&self) -> String {
        fs::read_to_string(Global::app_config()).expect("Error reading the YAML file")
    }

    fn add_text(
        &self,
        doc: &mut PdfDocumentReference,
        current_page: &mut PdfPageIndex,
        current_layer: &mut PdfLayerIndex,
        text: &str,
        y: &mut Mm,
        margin_left: &Mm,
        font: &IndirectFontRef,
        line_height: &Mm,
        size: f32,
    ) {
        if *y < Mm(20.0) {
            let (new_page, new_layer) = doc.add_page(Mm(210.0), Mm(297.0), "New page");
            *current_page = new_page;
            *current_layer = new_layer;
            *y = Mm(280.0);
        }

        let layer = doc.get_page(*current_page).get_layer(*current_layer);
        layer.use_text(text, size, *margin_left, *y, font);

        *y -= *line_height;
    }

    pub fn dump(&self) {
        let reports = ReportsHandlers::new(&self.path, self.interval, self.counter, None);

        let (mut doc, mut current_page, mut current_layer) =
            PdfDocument::new("Report dumps", Mm(210.0), Mm(297.0), "Layer 1");

        let font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();
        let font_header = doc.add_builtin_font(BuiltinFont::CourierBold).unwrap();

        let mg_top = Mm(280.0);
        let mg_left = Mm(10.0);
        let line_height = Mm(8.0);
        let mut y_position = mg_top;

        self.add_text(&mut doc, &mut current_page, &mut current_layer, "Final report", &mut y_position, &mg_left, &font_header, &line_height, 12.0);
        self.add_text(&mut doc, &mut current_page, &mut current_layer, &format!("Path: {}", &self.path), &mut y_position, &mg_left, &font, &line_height, 8.0);
        self.add_text(&mut doc, &mut current_page, &mut current_layer, &format!("Interval: {} seconds", &self.interval), &mut y_position, &mg_left, &font, &line_height, 8.0);
        self.add_text(&mut doc, &mut current_page, &mut current_layer, &format!("Total number of dumps: {}", &self.counter), &mut y_position, &mg_left, &font, &line_height, 8.0);

        if let Some((last_dump, size)) = reports.get_most_recent_sql_file(&self.path) {
            self.add_text(&mut doc, &mut current_page, &mut current_layer, &format!("Last dump: {} ({})", last_dump, size), &mut y_position, &mg_left, &font, &line_height, 8.0);

            if let Some(tables) = reports.extract_table_names(&last_dump) {
                self.add_text(&mut doc, &mut current_page, &mut current_layer, "Tables dumped:", &mut y_position, &mg_left, &font_header, &line_height, 12.0);

                for table in tables {
                    self.add_text(&mut doc, &mut current_page, &mut current_layer, &format!("- {}", table), &mut y_position, &mg_left, &font, &line_height, 8.0);
                }
            } else {
                self.add_text(&mut doc, &mut current_page, &mut current_layer, "No tables found in the dump.", &mut y_position, &mg_left, &font, &line_height, 8.0);
            }
        }

        self.add_text(&mut doc, &mut current_page, &mut current_layer, "Settings:", &mut y_position, &mg_left, &font_header, &line_height, 12.0);
        
        for line in self.read_yaml_as_text().lines() {
            if line.is_empty() { continue }
            self.add_text(&mut doc, &mut current_page, &mut current_layer, line, &mut y_position, &mg_left, &font, &line_height, 8.0);
        }

        let mut pdf = BufWriter::new(File::create(&self.file).expect("Error creating PDF"));
        doc.save(&mut pdf).expect("Error saving PDF");

        ReportAlerts::success_pdf(&self.file);
    }

}
