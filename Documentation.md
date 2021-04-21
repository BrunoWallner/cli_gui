# Documentation
If you want to create a empty window you can do that like that:

```Rust
use cli_gui::*;

fn main() {
    // creates main window
    let mut window = Window::new(Position::new(1, 0), Size::new(100, 50));
    window.set_to_main();

    // writes text to window
    window.write(Position::new(5, 5), "Hello".to_string(), Color::new(100, 255, 50));

    // decorates window with title and borders
    window.decorate();

    // renders window to terminal
    window.render();
    
    // halts the programm to then quit
    window.read_line(Position::new(10, 10), "input: ", Color::new(100, 100, 100), true);
    window.quit();
}
```
![emptywindow](https://github.com/BrunoWallner/cli_gui/blob/main/pictures/empty_window.png)

You can customize it like this:

```Rust
window.set_border_color(Color::new(0, 255, 0));
window.set_title("Window".to_string());
```
![customizedwindow](https://github.com/BrunoWallner/cli_gui/blob/main/pictures/customized_window.png)
Make sure that you do this before you use window.decorate()!

Or you can write text into the window like:
```Rust
window.write(Position::new(5, 5), "Hello".to_string(), Color::new(100, 255, 50));
```
Later it even will be possible to draw lines.

You can even read user input from a sub window
```Rust
// reads line, blocking
input = window.read_line(Position::new(10, 10), "input: ", Color::new(100, 100, 100), true);

// reads char if one is pressed, waits 1ms everytime for keypress when this functions is called
input = window.read_char();
```

you can clear the window with window.clear() and the entire terminal with clear_terminal()

Here is the enitre code:
```Rust
    // creates main window
    let mut window = Window::new(Position::new(1, 0), Size::new(100, 50));
    window.set_to_main();

    // writes text to window
    window.write(Position::new(5, 5), "Hello".to_string(), Color::new(100, 255, 50));

    // decorates the window
    window.set_border_color(Color::new(0, 255, 0));
    window.set_title("Window".to_string());
    
    // decorates window with title and borders
    window.decorate();

    // renders window to terminal
    window.render();

    // halts the programm
    window.read_line(Position::new(10, 10), "input: ", Color::new(100, 100, 100), true);
    window.quit();
}
```
![finalwindow](https://github.com/BrunoWallner/cli_gui/blob/main/pictures/final_window.png)
