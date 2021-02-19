use crate::pieces::*;
use crate::{get_rook_handle};
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_mod_picking::{Group, PickState, PickableMesh};

const ROWS: u8 = 8;
const COLS: u8 = 8;

#[derive(Debug)]
pub struct Square {
    pub x: u8,
    pub y: u8,
}

impl Square {
    fn is_white(&self) -> bool {
        (self.x + self.y + 1) % 2 == 0
    }
}

#[derive(Default)]
struct SelectedSquare {
    entity: Option<Entity>,
}

#[derive(Default)]
struct SelectedPiece {
    entity: Option<Entity>,
}

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SelectedSquare>()
            .init_resource::<SelectedPiece>()
            .init_resource::<PlayerTurn>()
            .add_event::<ResetSelectedEvent>()
            .add_startup_system(create_board.system())
            .add_system(color_squares.system())
            .add_system(select_square.system())
            .add_system(move_piece.system())
            .add_system(select_piece.system())
            .add_system(despawn_taken_pieces.system())
            .add_system(reset_selected.system());
    }
}

/**
 * @brief adds board mesh plane, spawns ROWS * COLS squares with 
 *  pickable mesh enabled
 * */
pub fn create_board(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add meshes
    let mesh = meshes.add(Mesh::from(shape::Plane { size: 1. }));

    // Spawn 64 squares
    for i in 0..ROWS {
        for j in 0..COLS {
            commands
                .spawn(PbrBundle {
                    mesh: mesh.clone(),

                    // Change material according to position to get alternating pattern
                    material: if (i + j + 1) % 2 == 0 {
                        materials.add(Color::rgb(1., 0.9, 0.9).into())
                    } else {
                        materials.add(Color::rgb(0., 0.1, 0.1).into())
                    },
                    transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
                    ..Default::default()
                })
                .with(PickableMesh::default())
                .with(Square { x: i, y: j });
        }
    }
}

/**
 * @brief set square colors based on albedo
 * */
fn color_squares(
    pick_state: Res<PickState>,
    selected_square: Res<SelectedSquare>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Square, &Handle<StandardMaterial>)>,
) {
    // Get entity under the cursor, if there is one
    let top_entity = if let Some((entity, _intersection)) = pick_state.top(Group::default()) {
        Some(*entity)
    } else {
        None
    };

    for (entity, square, material_handle) in query.iter() {
        // Get the actual material
        let material = materials.get_mut(material_handle).unwrap();

        // Change the material color
        material.albedo = if Some(entity) == top_entity {
            Color::rgb(0.8, 0.3, 0.3)
        } else if Some(entity) == selected_square.entity {
            Color::rgb(0.9, 0.1, 0.1)
        } else if square.is_white() {
            Color::rgb(1., 0.9, 0.9)
        } else {
            Color::rgb(0., 0.1, 0.1)
        };
    }
}

/**
 * @brief handle the left mouse click event, set the selected square entity 
 *  or handle square deselection
 * */
fn select_square(
    pick_state: Res<PickState>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    squares_query: Query<&Square>,
) {
    // Only run if the left button is pressed
    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }

    // Get the square under the cursor and set it as the selected
    if let Some((square_entity, _intersection)) = pick_state.top(Group::default()) {
        // Get the actual square. This ensures it exists and is a square. Not really needed
        if let Ok(_square) = squares_query.get(*square_entity) {
            // Mark it as selected
            selected_square.entity = Some(*square_entity);
        }
    } else {
        // Player clicked outside the board, deselect everything
        selected_square.entity = None;
        selected_piece.entity = None;
    }
}

/**
 * @brief select the piece in the selected square
 * */
fn select_piece(
    selected_square: ChangedRes<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    turn: Res<PlayerTurn>,
    squares_query: Query<&Square>,
    pieces_query: Query<(Entity, &Piece)>,
) {
    let square_entity = if let Some(entity) = selected_square.entity {
        entity
    } else {
        return;
    };

    let square = if let Ok(square) = squares_query.get(square_entity) {
        square
    } else {
        return;
    };

    if selected_piece.entity.is_none() {
        // Select the piece in the currently selected square
        for (piece_entity, piece) in pieces_query.iter() {
            if piece.x == square.x && piece.y == square.y && piece.color == turn.0 {
                // piece_entity is now the entity in the same square
                selected_piece.entity = Some(piece_entity);
                break;
            }
        }
    }
}

/**
 * @brief return standard material based on the piece color
 * */
fn get_material(mut materials: ResMut<Assets<StandardMaterial>>, color: &PieceColor) -> Handle<StandardMaterial> {
    match color{
        PieceColor::Black => return materials.add(Color::rgb(0., 0.2, 0.2).into()),
        PieceColor::White => return materials.add(Color::rgb(1., 0.8, 0.8).into()),
    }
}

/**
 * @brief
 * */
fn check_despawn(
    commands: &mut Commands, 
    square: &Square, 
    piece: &bevy::prelude::Mut<Piece>, 
    pieces_entity_vec: Vec<(bevy::prelude::Entity, Piece)>){
    for (other_entity, other_piece) in pieces_entity_vec {
        if other_piece.x == square.x
            && other_piece.y == square.y
            && other_piece.color != piece.color
        {
            // Mark the piece as taken
            commands.insert_one(other_entity, Taken);
        }
    }
}

fn move_piece(
    commands: &mut Commands,
    selected_square: ChangedRes<SelectedSquare>,
    selected_piece: Res<SelectedPiece>,
    mut turn: ResMut<PlayerTurn>,
    squares_query: Query<&Square>,
    mut pieces_query: Query<(Entity, &mut Piece)>,
    mut reset_selected_event: ResMut<Events<ResetSelectedEvent>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
    let square_entity = if let Some(entity) = selected_square.entity {
        entity
    } else {
        return;
    };

    let square = if let Ok(square) = squares_query.get(square_entity) {
        square
    } else {
        return;
    };

    if let Some(selected_piece_entity) = selected_piece.entity {
        let pieces_vec = pieces_query.iter_mut().map(|(_, piece)| *piece).collect();
        let pieces_entity_vec = pieces_query
            .iter_mut()
            .map(|(entity, piece)| (entity, *piece))
            .collect::<Vec<(Entity, Piece)>>();
        // Move the selected piece to the selected square
        let mut piece =
            if let Ok((_piece_entity, piece)) = pieces_query.get_mut(selected_piece_entity) {
                piece
            } else {
                return;
            };


        if piece.is_move_valid((square.x, square.y), pieces_vec) {

            // perform castling
            match piece.piece_type{
                PieceType::King =>{
                    if piece.x == square.x && piece.y == 4 && square.y == 6{

                        // find corresponding rook, take it out and respawn it to its new position
                        for (other_entity, other_piece) in &pieces_entity_vec{
                            if other_piece.x == square.x &&
                                other_piece.y == 7 &&
                                other_piece.piece_type == PieceType::Rook &&
                                other_piece.color == piece.color{

                                // move king to its new position
                                piece.y = 6;

                                // take the castle rook out
                                commands.insert_one(*other_entity, Taken);

                                // respawn rook at its new position
                                spawn_rook(commands, get_material(materials, &piece.color), piece.color, get_rook_handle(&asset_server),
                                    (piece.x, 5)
                                );

                                turn.change();
                                return;
                            }
                        }
                    }
                    else if piece.x == square.x && piece.y == 4 && square.y == 2{
                        // find corresponding rook, take it out and respawn it to its new position
                        for (other_entity, other_piece) in &pieces_entity_vec{
                            if other_piece.x == square.x &&
                                other_piece.y == 0 &&
                                other_piece.piece_type == PieceType::Rook &&
                                other_piece.color == piece.color{

                                    // take the castle rook out
                                    commands.insert_one(*other_entity, Taken);

                                    // respawn rook at its new position
                                    spawn_rook(
                                        commands,
                                        match piece.color{
                                            PieceColor::Black => materials.add(Color::rgb(0., 0.2, 0.2).into()),
                                            PieceColor::White => materials.add(Color::rgb(1., 0.8, 0.8).into()),
                                        },
                                        piece.color,
                                        asset_server.load("models/chess_kit/pieces.glb#Mesh5/Primitive0"),
                                        (piece.x, 3)
                                    );
                                }
                        }
                    }
                }
                _ => {}
            }

            // Check if a piece of the opposite color exists in this square and despawn it
            check_despawn(commands, square, &piece, pieces_entity_vec);

            // Move piece
            piece.x = square.x;
            piece.y = square.y;

            // Change turn
            turn.change();
        }

        reset_selected_event.send(ResetSelectedEvent);
    }
}

struct ResetSelectedEvent;

fn reset_selected(
    mut event_reader: Local<EventReader<ResetSelectedEvent>>,
    events: Res<Events<ResetSelectedEvent>>,
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
) {
    for _event in event_reader.iter(&events) {
        selected_square.entity = None;
        selected_piece.entity = None;
    }
}

struct Taken;

fn despawn_taken_pieces(
    commands: &mut Commands,
    mut app_exit_events: ResMut<Events<AppExit>>,
    query: Query<(Entity, &Piece, &Taken)>,
) {
    for (entity, piece, _taken) in query.iter() {
        // If the king is taken, we should exit
        if piece.piece_type == PieceType::King {
            println!(
                "{} won! Thanks for playing!",
                match piece.color {
                    PieceColor::White => "Black",
                    PieceColor::Black => "White",
                }
            );
            app_exit_events.send(AppExit);
        }

        // Despawn piece and children
        commands.despawn_recursive(entity);
    }
}

// Turns ======================================================================================== //
pub(crate) struct PlayerTurn(pub(crate) PieceColor);
impl Default for PlayerTurn {
    fn default() -> Self {
        Self(PieceColor::White)
    }
}

impl PlayerTurn {
    fn change(&mut self) {
        self.0 = match self.0 {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        }
    }
}
