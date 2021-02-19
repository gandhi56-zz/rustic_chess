use bevy::prelude::*;

pub fn get_king_handle(asset_server: &Res<AssetServer>,) -> Handle<Mesh>{
    asset_server.load("models/chess_kit/pieces.glb#Mesh0/Primitive0")
}
pub fn get_queen_handle(asset_server: &Res<AssetServer>,) -> Handle<Mesh>{
    asset_server.load("models/chess_kit/pieces.glb#Mesh7/Primitive0")
}

pub fn get_rook_handle(asset_server: &Res<AssetServer>,) -> Handle<Mesh>{
    asset_server.load("models/chess_kit/pieces.glb#Mesh5/Primitive0")
}
pub fn get_bishop_handle(asset_server: &Res<AssetServer>,) -> Handle<Mesh>{
    asset_server.load("models/chess_kit/pieces.glb#Mesh6/Primitive0")
}
pub fn get_knight1_handle(asset_server: &Res<AssetServer>,) -> Handle<Mesh>{
    asset_server.load("models/chess_kit/pieces.glb#Mesh3/Primitive0")
}

pub fn get_knight2_handle(asset_server: &Res<AssetServer>,) -> Handle<Mesh>{
    asset_server.load("models/chess_kit/pieces.glb#Mesh4/Primitive0")
}
pub fn get_pawn_handle(asset_server: &Res<AssetServer>,) -> Handle<Mesh>{
    asset_server.load("models/chess_kit/pieces.glb#Mesh2/Primitive0")
}

pub fn get_king_cross_handle(asset_server: &Res<AssetServer>,) -> Handle<Mesh>{
    asset_server.load("models/chess_kit/pieces.glb#Mesh1/Primitive0")
}