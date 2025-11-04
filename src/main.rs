mod ui;

use slint::ComponentHandle;
use ui::MainWindow;

fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;
    main_window.run()
}
