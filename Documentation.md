# Documentation
If you want to create a empty window you can do that like that:

```Rust
use std::io::{Write, stdout};
use cli_gui::*;

fn main() {
        // creates main window
        let mut window = Window::new(Size::new(120, 45)); // <-- Size

        // creates sub window
        let mut login_window = SubWindow::new(Position::new(35, 15), Size::new(50, 15));

        // draws sub window into main window
        window.write_window(&login_window);

        // renders main window
        window.render();
        stdout().flush()
            .expect("failed to flush stdout :(");
}
```
![emptywindow](https://github.com/BrunoWallner/cli_gui/blob/main/pictures/empty_window.png)

You can customize it like this:

```Rust
login_window.set_border_color(1); // <- sets border color
login_window.set_title("Login".to_string()); // <- sets title of sub window
login_window.set_title_color(5); // <- sets title color
login_window.move_window(Position::new(12, 3)); // <- moves the window
```
![customizedwindow](https://github.com/BrunoWallner/cli_gui/blob/main/pictures/customized_window.png)
Make sure that you do this befor writing the sub window into the main window ( window.write_window(&login_window) )!

Or you can write text into the window like:
```Rust
login_window.write(Position::new(5, 5), "Moin Meister".to_string(), 3);
login_window.write(Position::new(25, 9), "Test".to_string(), 0);
```

You can even read user input from a sub window
```Rust
let input = login_window.input(Position::new(7, 10), "input: ");
```

Make sure you do this after rendering the main window because input() is blocking :(

you can clear the window with window.clear()

## Color-codes
number	| color
------	| ------
0	| white
1	| red
2	| green
3	| blue
4	| cyan
5	| magenta

there will be more available in later releases

Here is the enitre code:
```Rust
use std::io::{Write, stdout};
use cli_gui::*;

fn main() {
        // creates main window
        let mut window = Window::new(Size::new(120, 45));

        // creates sub window
        let mut login_window = SubWindow::new(Position::new(35, 15), Size::new(50, 15));

        // draws sub window into main window
        login_window.set_border_color(1); // <- sets border color
        login_window.set_title("Login".to_string()); // <- sets title of sub window
        login_window.set_title_color(5); // <- sets title color
        login_window.move_window(Position::new(12, 3)); // <- moves the window

        login_window.write(Position::new(5, 5), "Moin Meister".to_string(), 3);
        login_window.write(Position::new(25, 9), "Test".to_string(), 0);

        window.write_window(&login_window);

        // renders main window
        window.render();
        stdout().flush()
            .expect("failed to flush stdout :(");

        let input = login_window.input(Position::new(7, 10), "input: ");
        login_window.write(Position::new(20, 5), input, 3);
        window.write_window(&login_window);


        // renders main window
        window.render();
        stdout().flush()
            .expect("failed to flush stdout :(");
}
```
![finalwindow](https://github.com/BrunoWallner/cli_gui/blob/main/pictures/final_window.png)
