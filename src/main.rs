use bevy::{asset::LoadState, prelude::*};
use ldtk_rs::{Root, TilesetDefinition};
use std::collections::HashMap;

macro_rules! files {
    (AUTOLAYERS_1_BASIC) => {
        "../assets/AutoLayers_1_basic.ldtk"
    };
    (AUTOLAYERS_2_STAMPS) => {
        "../assets/AutoLayers_2_stamps.ldtk"
    };
    (AUTOLAYERS_3_MOSAIC) => {
        "../assets/AutoLayers_3_Mosaic.ldtk"
    };
    (AUTOLAYERS_4_ADVANCED) => {
        "../assets/AutoLayers_4_Advanced.ldtk"
    };
    (ENTITIES) => {
        "../assets/Entities.ldtk"
    };
    (TYPICAL_2D_PLATFORMER_EXAMPLE) => {
        "../assets/Typical_2D_platformer_example.ldtk"
    };
    (TYPICAL_TOPDOWN_EXAMPLE) => {
        "../assets/Typical_TopDown_example.ldtk"
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json = include_str!(files!(ENTITIES));
    let mut data = Root::new(json)?;

    dbg!(data.bg_color);
    println!("...");
    // dbg!(data.default_grid_size);
    // dbg!(data.default_pivot_x);
    // dbg!(data.export_tiled);
    // dbg!(data.json_version);
    // dbg!(data.minify_json);
    // dbg!(data.defs);

    // dbg!(data.defs.entities.len());
    // let entity = &data.defs.entities[0];
    // dbg!(entity);

    // dbg!(data.defs.enums.len());
    // dbg!(data.defs.external_enums.len());

    // dbg!(data.defs.layers.len());

    // dbg!(data.defs.tilesets.len());
    let tileset = &data.defs.tilesets[0];
    dbg!(tileset.padding);
    dbg!(tileset.px_hei);
    dbg!(tileset.px_wid);
    dbg!(tileset.spacing);
    dbg!(tileset.tile_grid_size);
    dbg!(tileset.uid);
    dbg!(&tileset.rel_path);
    println!("...");

    // dbg!(data.levels);
    dbg!(data.levels.len());
    let level = &data.levels[0];
    dbg!(&level.identifier);
    dbg!(level.px_hei);
    dbg!(level.px_wid);
    dbg!(level.uid);
    println!("...");

    dbg!(level.layer_instances.len());
    println!("...");

    for (_index, layer) in level.layer_instances.iter().enumerate() {
        dbg!(layer.__c_hei);
        dbg!(layer.__c_wid);
        dbg!(layer.__grid_size);
        dbg!(&layer.__identifier);
        dbg!(layer.__opacity);
        dbg!(layer.__px_total_offset_x);
        dbg!(layer.__px_total_offset_y);
        dbg!(&layer.__type);
        if let Some(ref auto_layer_tiles) = layer.auto_layer_tiles {
            if dbg!(auto_layer_tiles.len()) > 0 {
                let auto_layer_tile = &auto_layer_tiles[0];
                dbg!(auto_layer_tile);
            }
        }
        if let Some(ref entity_instances) = layer.entity_instances {
            if dbg!(entity_instances.len()) > 0 {
                let entity_instance = &entity_instances[0];
                dbg!(entity_instance);
            }
        }
        if let Some(ref grid_tiles) = layer.grid_tiles {
            if dbg!(grid_tiles.len()) > 0 {
                let grid_tile = &grid_tiles[0];
                dbg!(grid_tile);
            }
        }
        if let Some(ref int_grid) = layer.int_grid {
            if dbg!(int_grid.len()) > 0 {
                let int_grid_cell = &int_grid[0];
                dbg!(int_grid_cell);
            }
        }
        dbg!(layer.px_offset_x);
        dbg!(layer.px_offset_y);
        dbg!(layer.layer_def_uid);
        println!("...");
    }

    // let layer1 = &level.layer_instances[1];
    // dbg!(layer1.level_id);

    let level = data.levels.remove(0); // TODO: more than one level `data.levels`, load on demand

    App::build() // TODO: asset loader
        .add_resource(data.defs.tilesets) // TODO: lazy-load
        .add_resource(level.layer_instances)
        .init_resource::<TilesetHandles>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(load_tileset_textures)
        .add_startup_system(setup)
        .add_system(load_atlas_textures)
        .add_system(spawn_sprites)
        .add_system(animate_sprites)
        .run();

    Ok(())
}

#[derive(Default)]
pub struct TilesetHandles {
    texture_handles: HashMap<i32, Handle<Texture>>,
    loaded: bool,
    atlas_handles: HashMap<i32, Handle<TextureAtlas>>,
}

fn setup(commands: &mut Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn load_tileset_textures(
    tilesets: Res<Vec<TilesetDefinition>>,
    mut tileset_handles: ResMut<TilesetHandles>,
    asset_server: Res<AssetServer>,
) {
    // load the folder and create handles to the files
    for tileset in tilesets.iter() {
        tileset_handles.texture_handles.insert(
            tileset.uid,
            asset_server.load::<_, _>(tileset.rel_path.as_str()),
        );
    }
}

fn load_atlas_textures(
    mut tileset_handles: ResMut<TilesetHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    textures: Res<Assets<Texture>>,
    tilesets: Res<Vec<TilesetDefinition>>,
) {
    // don't run this whole system if we've already loaded the atlas (i.e. run once only, when loaded)
    if tileset_handles.loaded {
        return;
    }

    // but if we haven't...
    // wait until texture assets are loaded
    if let LoadState::Loaded = asset_server.get_group_load_state(
        tileset_handles
            .texture_handles
            .iter()
            .map(|(_, handle)| handle.id),
    ) {
        dbg!(tileset_handles.texture_handles.len());
        // padding left | tile | spacing | tile | padding right
        // p | s | x | s | p
        // n = 2
        // inner_dimension = dimension - 2 * padding; where dimension is height or width
        // tile.(n) + spacing.(n-1) = inner_dimension
        // tile.n + spacing.n - spacing = inner_dimension
        // tile.n + spacing.n = inner_dimension + spacing
        // n.(tile + spacing) = inner_dimension + spacing
        // n = (inner_dimension + spacing) / (tile + spacing)
        // n = (dimension - 2 * padding + spacing) / (tile + spacing)
        for tileset in tilesets.iter() {
            let padding = dbg!(tileset.padding);
            let height = dbg!(tileset.px_hei);
            let width = dbg!(tileset.px_wid);
            let spacing = dbg!(tileset.spacing);
            let tile = dbg!(tileset.tile_grid_size);
            let uid = dbg!(tileset.uid);
            let tileset_handle = tileset_handles.texture_handles.get(&uid).unwrap();
            let columns = dbg!((width - 2 * padding + spacing) / (tile + spacing));
            let rows = dbg!((height - 2 * padding + spacing) / (tile + spacing));

            let texture_atlas = TextureAtlas::from_grid_with_padding(
                tileset_handle.clone(),
                Vec2::splat(tile as f32),
                columns as usize,
                rows as usize,
                Vec2::splat(spacing as f32),
            );
            let texture_atlas_handle = texture_atlases.add(texture_atlas);
            tileset_handles
                .atlas_handles
                .insert(uid, texture_atlas_handle);
        }

        tileset_handles.loaded = true;
    }
}

fn spawn_sprites(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    tileset_handles: Res<TilesetHandles>,
    tilesets: Res<Vec<TilesetDefinition>>,
    mut done: Local<bool>,
) {
    // don't run this whole system if we've already loaded the atlas (i.e. run once only, when loaded)
    if !tileset_handles.loaded || *done {
        return;
    }

    let mut cursor = Vec2::new(640.0, 360.0);
    const SCALE: f32 = 4.0;
    for tileset in tilesets.iter() {
        let height = tileset.px_hei;
        let width = tileset.px_wid;
        let tile = tileset.tile_grid_size;
        let uid = tileset.uid;
        let size = Vec2::new(width as f32, height as f32);
        cursor -= size;
        let position = cursor + 0.5 * size;
        let position2_x = position.x - 0.5 * width as f32 - SCALE * tile as f32 * 0.5;
        let mut transform2 =
            Transform::from_translation(Vec2::new(position2_x, position.y).extend(0.0));
        transform2.scale = Vec2::new(-SCALE, SCALE).extend(0.0); // X is reversed
                                                                 // for now, just spawn one sprite per tile
                                                                 // set up a scene to display our texture atlas
        let texture_handle = tileset_handles.texture_handles.get(&uid).unwrap();
        // draw the atlas (tilesheet) itself
        commands.spawn(SpriteBundle {
            material: materials.add(texture_handle.clone().into()),
            transform: Transform::from_translation(position.extend(0.0)),
            ..Default::default()
        });
        // draw one sprite (tile) from the atlas (tilesheet)
        let texture_atlas_handle = tileset_handles.atlas_handles.get(&uid).unwrap();
        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                transform: transform2,
                ..Default::default()
            })
            .with(Timer::from_seconds(0.5, true));
    }

    *done = true;
}

fn animate_sprites(
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        if timer.finished {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }
    }
}
