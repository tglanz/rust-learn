use engine::tiles::Tile;
use engine::game_state::GameState;
use engine::snake;

enum Element {
    TileValue(Tile),
    Snake(bool)
}

fn get_render_string(element: &Element) -> &str {
    match element {
        Element::TileValue(Tile::Empty) => " ",
        Element::TileValue(Tile::Food) => "~",
        Element::TileValue(Tile::Occupied) => "#",
        Element::Snake(true) => "0",
        Element::Snake(false) => "O",
    }
}

pub fn render(game_state: &GameState) {
    let mut content = "".to_owned();

    content.push_str(&format!("{}[2J\n", 27 as char));
    content.push_str(&format!("Score {}\n", snake::count(game_state.get_snake())));

    for (idx, tile) in game_state.get_tiles().iter().enumerate() {
        if idx % game_state.get_terrain_size().get_columns() as usize == 0 {
            content.push_str("\n");
        }

        let idx_usize_ref = &(idx as i16);
        let snake_ref = &game_state.get_snake();
        if snake::is_occupy_terrain_index(snake_ref, idx_usize_ref) {
            content.push_str(
                get_render_string(&Element::Snake(
                    snake::is_head(snake_ref, idx_usize_ref)
                ))
            );
        } else {
            content.push_str(get_render_string(&Element::TileValue(*tile)));
        }

    }

    println!("{}", content);
}