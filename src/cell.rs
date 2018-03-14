use prettytable::cell::Cell;
use term::{color, Attr};

pub struct LayoutCell {
    cell: Cell,
}

impl LayoutCell {
    pub fn new() -> LayoutCell {
        LayoutCell {
            cell: Cell::new(""),
        }
    }

    pub fn set(&mut self, text: &str) -> &mut LayoutCell {
        self.cell = Cell::new(text);
        self
    }

    pub fn percent_color(
        &mut self,
        (text, is_positive): (String, bool),
    ) -> &mut LayoutCell {
        self.set(&text);

        if is_positive {
            self.green();
        } else {
            self.red();
        };

        self
    }

    pub fn set_and_build(&mut self, text: &str) -> Cell {
        self.set(text);
        self.cell.clone()
    }

    pub fn build(&self) -> Cell {
        self.cell.clone()
    }

    pub fn bold(&mut self) -> &mut LayoutCell {
        self.cell = self.cell.clone().with_style(Attr::Bold);
        self
    }

    pub fn yellow(&mut self) -> &mut LayoutCell {
        self.cell = self.cell
            .clone()
            .with_style(Attr::ForegroundColor(color::YELLOW));
        self
    }

    pub fn blue(&mut self) -> &mut LayoutCell {
        self.cell = self.cell
            .clone()
            .with_style(Attr::ForegroundColor(color::BLUE));
        self
    }
    pub fn green(&mut self) -> &mut LayoutCell {
        self.cell = self.cell
            .clone()
            .with_style(Attr::ForegroundColor(color::GREEN));
        self
    }

    pub fn red(&mut self) -> &mut LayoutCell {
        self.cell = self.cell
            .clone()
            .with_style(Attr::ForegroundColor(color::RED));
        self
    }
}
