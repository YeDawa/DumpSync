use printpdf::*;

use std::{
    fs::File,
    io::BufWriter,
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

    fn add_text(&self, layer: &PdfLayerReference, text: &str, y: &mut Mm, margin_left: &Mm, font: &IndirectFontRef, line_height: &Mm, size: f32) {
        layer.use_text(text, size, *margin_left, *y, font);
        *y -= *line_height;
    }

    pub fn dump(&self) {
        let (doc, page1, layer1) = PdfDocument::new("Report dumps", Mm(210.0), Mm(297.0), "Camada 1");
        let current_layer = doc.get_page(page1).get_layer(layer1);

        let font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();
        let font_header = doc.add_builtin_font(BuiltinFont::CourierBold).unwrap();

        let line_height = Mm(8.0);
        let margin_top = Mm(280.0);
        let margin_left = Mm(10.0);
        let mut y_position = margin_top;

        let x_pos = Mm(10.0);
        let y_pos = Mm(20.0);

        let link_text = format!("Generated using {}", Global::APP_NAME);
        current_layer.use_text(link_text, 16.0, x_pos, y_pos, &font);

        let link_annotation = LinkAnnotation::new(
            printpdf::Rect::new(x_pos, y_pos - Mm(2.0), x_pos + Mm(100.0), y_pos + Mm(8.0)), // Define a área clicável
            Some(printpdf::BorderArray::default()),
            Some(printpdf::ColorArray::default()),
            printpdf::Actions::uri(Global::APP_HOMEPAGE.to_string()),
            Some(printpdf::HighlightingMode::Invert),
        );
    
        current_layer.add_link_annotation(link_annotation);

        self.add_text(&current_layer, "Final report", &mut y_position, &margin_left, &font_header, &line_height, 12.0);

        self.add_text(&current_layer, &format!("Path: {}", &self.path), &mut y_position, &margin_left, &font, &line_height, 8.0);
        self.add_text(&current_layer, &format!("Interval: {} seconds", &self.interval), &mut y_position, &margin_left, &font, &line_height, 8.0);
        self.add_text(&current_layer, &format!("Total number of dumps: {}", &self.counter), &mut y_position, &margin_left, &font, &line_height, 8.0);

        if let Some((last_dump, size)) = ReportsHandlers.get_most_recent_sql_file(&self.path) {
            self.add_text(&current_layer, &format!("Last dump: {} ({})", last_dump, size), &mut y_position, &margin_left, &font, &line_height, 8.0);

            if let Some(tables) = ReportsHandlers.extract_table_names(&last_dump) {
                self.add_text(&current_layer, "Tables dumped:", &mut y_position, &margin_left, &font_header, &line_height, 12.0);

                for table in tables {
                    self.add_text(&current_layer, &format!("- {}", table), &mut y_position, &margin_left, &font, &line_height, 8.0);
                }
            } else {
                self.add_text(&current_layer, "No tables found in the dump.", &mut y_position, &margin_left, &font, &line_height, 8.0);
            }
        }

        let mut pdf = BufWriter::new(File::create(&self.file).expect("Error creating PDF"));
        doc.save(&mut pdf).expect("Error saving PDF");

        ReportAlerts::success_pdf(&self.file);
    }

}
