use macroquad::prelude::*;

// Types to have a better overview for constants
type TypeMode = i8;
type TypeError = i8;

// Mode types
//const MODE_TITLE_SCREEN: TypeMode = 0;
//const MODE_MENU_MAIN: TypeMode = 1;
//const MODE_MENU_OPTIONS: TypeMode = 2;
//const MODE_MENU_GALLERY: TypeMode = 3;
//const MODE_MENU_SAVE_FILES: TypeMode = 4;
//const MODE_INGAME_TOWN: TypeMode = 5;
//const MODE_INGAME_DUNGEON: TypeMode = 6;
//const MODE_INGAME_ANIMATION: TypeMode = 7;


#[macroquad::main("Dungeon Escape (WiP title)")]
async fn main() {
    let mut test_area: Vec<Vec<i8>> = get_floor(10);
    println!("{:#?}", test_area);
//    loop {
//        println!("Hello, world!");
//        next_frame().await;
//    }
    
}

// returns floor without walls at the moment
fn get_floor(size: i8) -> Vec<Vec<i8>> {
    let mut floor = vec![];
    for row in 0..size {
        floor.push(vec![]);
        for _coloumn in 0..size {
            floor[row as usize].push(0);
        }
    }
    return floor;
}

fn check_movement() -> () {
    if macroquad::input::is_key_pressed(miniquad::KeyCode::W) {

    }
}