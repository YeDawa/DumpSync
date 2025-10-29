use open;

use crate::ui::errors_alerts::ErrorsAlerts;

pub struct Open {
    item: String
}

impl Open {

    pub fn new(item: &str) -> Self {
        Self {
            item: item.to_owned()
        }
    }

    pub fn link(&self) {
        if open::that(&self.item).is_err() {
            ErrorsAlerts::open_link();
        }
    }

}