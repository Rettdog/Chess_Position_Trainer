use fltk::{
    app::{self, *},
    button::*,
    dialog::{self, BeepType},
    draw,
    misc::Progress,
    window::*,
};
use rand::{self, Rng};

static mut RED_COORDS: Vec<(i32, i32)> = vec![];
static mut IS_REVERSED: bool = false;
static mut TARGET_SQUARE: (i32, i32) = (3, 4);
static mut PROGRESS_VALUE: f64 = 0.0;

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Base);
    let mut wind = Window::new(100, 100, 900, 900, "Chess Position Trainer");
    wind.set_color(Color::from_rgb(200, 200, 180));

    let mut white_start = Button::new(750, 100, 120, 30, "Train White");
    white_start.set_callback(|| unsafe {
        IS_REVERSED = false;
        PROGRESS_VALUE = 0.0;
        let new_x = rand::thread_rng().gen_range(1..=8);
        let new_y = rand::thread_rng().gen_range(1..=8);
        println!("{} {}", new_x, new_y);
        TARGET_SQUARE = (new_x, new_y);
    });

    let mut black_start = Button::new(750, 140, 120, 30, "Train Black");
    black_start.set_callback(|| unsafe {
        IS_REVERSED = true;
        PROGRESS_VALUE = 0.0;
        let new_x = rand::thread_rng().gen_range(1..=8);
        let new_y = rand::thread_rng().gen_range(1..=8);
        println!("{} {}", new_x, new_y);
        TARGET_SQUARE = (new_x, new_y);
    });

    let mut progress = Progress::new(750, 200, 120, 30, "");
    progress.set_minimum(0.0);
    progress.set_maximum(10.0);
    unsafe {
        progress.set_value(PROGRESS_VALUE);
    }

    wind.handle(|e| {
        if e == Event::Push && is_within_board(event_x(), event_y()) {
            unsafe {
                if convert_to_board(event_x(), event_y()) == TARGET_SQUARE {
                    PROGRESS_VALUE += 1.0;
                    //dialog::beep(BeepType::Default);
                } else {
                    PROGRESS_VALUE -= 1.0;
                }
                let new_x = rand::thread_rng().gen_range(1..=8);
                let new_y = rand::thread_rng().gen_range(1..=8);
                println!("{} {}", new_x, new_y);
                TARGET_SQUARE = (new_x, new_y);
            }
        }
        true
    });

    wind.draw(move || {
        draw::set_font(Font::CourierBold, 90);
        draw_text();
        draw::set_font(Font::CourierBold, 24);
        draw_board();
    });

    wind.make_resizable(true);
    wind.end();
    wind.show();
    while app.wait() {
        unsafe {
            if PROGRESS_VALUE < 0.0 {
                PROGRESS_VALUE = 0.0;
            }
            progress.set_value(PROGRESS_VALUE);
            PROGRESS_VALUE -= 0.005;
        }

        wind.redraw();

        app::sleep(0.02);
    }

    app.run().unwrap();
}

fn is_within_board(x: i32, y: i32) -> bool {
    let size: i32 = 90;
    x < (8 * size) && size < y && y < (9 * size)
}

fn convert_to_board(x: i32, y: i32) -> (i32, i32) {
    let size: i32 = 90;
    let mut column = (x - x % size) / size + 1;
    let mut row = 9 - (y - y % size) / size;
    unsafe {
        if IS_REVERSED {
            column = 8 - ((x - x % size) / size);
            row = (y - y % size) / size;
        }
    }
    println!(
        "x:{} y:{} mouseX:{} mouseY:{}",
        column,
        row,
        event_x(),
        event_y()
    );
    (column, row)
}

fn convert_board_to_space(x: i32, y: i32) -> String {
    let mut letters = [
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "d".to_string(),
        "e".to_string(),
        "f".to_string(),
        "g".to_string(),
        "h".to_string(),
    ];
    let out = &mut letters[(x - 1) as usize];
    out.push_str(&(y).to_string());
    let act_out = &out;
    act_out.to_string()
}

fn draw_text() {
    //unsafe { draw::draw_text("test", 750, 300) }
    unsafe {
        let x = TARGET_SQUARE.0;
        let y = TARGET_SQUARE.1;
        draw::draw_text(&convert_board_to_space(x, y), 755, 400);
        if PROGRESS_VALUE >= 10.0 {
            draw::set_font(Font::CourierBold, 24);
            draw::draw_text("Complete", 750, 600);
        }
    }
}

fn draw_board() {
    let size: i32 = 90;
    let mut letters = [
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "d".to_string(),
        "e".to_string(),
        "f".to_string(),
        "g".to_string(),
        "h".to_string(),
    ];
    let mouse_x = &event_x();
    let mouse_y = &event_y();
    //draws white squares
    draw::set_draw_color(Color::from_rgb(238, 238, 210));
    for i in 0..4 {
        for j in 0..4 {
            draw::draw_rectf(i * 2 * size, j * 2 * size + size, size, size);
            draw::draw_rectf(i * 2 * size + size, j * 2 * size + 2 * size, size, size);
        }
    }
    //draws black squares
    draw::set_draw_color(Color::from_rgb(118, 150, 86));
    for i in 0..4 {
        for j in 0..4 {
            draw::draw_rectf(i * 2 * size + size, j * 2 * size + size, size, size);
            draw::draw_rectf(i * 2 * size, j * 2 * size + 2 * size, size, size);
        }
    }
    //draws red squares
    unsafe {
        for (i, j) in &RED_COORDS {
            draw::set_color_rgb(211, 98, 110);
            draw::draw_rectf(size * i - size, size * j, size, size);
        }
    }
    if mouse_x < &(8 * size) && &size < mouse_y && mouse_y < &(9 * size) {
        if event_is_click() {
            draw::set_color_rgb(211, 98, 110);
            draw::draw_rectf(
                mouse_x - mouse_x % size,
                mouse_y - mouse_y % size,
                size,
                size,
            );
            draw::set_color_rgb(0, 0, 0);

            let mut column = &mut letters[((mouse_x - mouse_x % size) / size) as usize];
            let mut row = 9 - ((mouse_y - mouse_y % size) / size);
            unsafe {
                if IS_REVERSED {
                    column = &mut letters[7 - ((mouse_x - mouse_x % size) / size) as usize];
                    row = (mouse_y - mouse_y % size) / size;
                }
            }

            column.push_str(&row.to_string());
            draw::draw_text2(
                column,
                mouse_x - mouse_x % size,
                mouse_y - mouse_y % size,
                size,
                size,
                Align::Center,
            );
        }
    }
    //draws black and white bars
    unsafe {
        if IS_REVERSED {
            draw::set_draw_color(Color::from_rgb(255, 255, 250));
            draw::draw_rectf(0, 0, 8 * size, size);
            draw::set_draw_color(Color::Black);
            draw::draw_rectf(0, 9 * size, 8 * size, size);
        } else {
            draw::set_draw_color(Color::from_rgb(10, 10, 10));
            draw::draw_rectf(0, 0, 8 * size, size);
            draw::set_draw_color(Color::from_rgb(255, 255, 250));
            draw::draw_rectf(0, 9 * size, 8 * size, size);
        }
    }
}
