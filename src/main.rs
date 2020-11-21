use bevy::{asset::LoadState, prelude::*, render::camera::Camera};
use ldtk_rs::{LayerDefinition, LayerInstance, Root, TilesetDefinition};
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
    (CUSTOM_BASIC) => {
        "../assets/custom_basic.ldtk"
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json = include_str!(files!(CUSTOM_BASIC));
    let mut data = Root::new(json)?;

    // dbg!(data.bg_color);
    // println!("...");
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

    dbg!(data.defs.layers.len());

    // dbg!(data.defs.tilesets.len());
    // let tileset = &data.defs.tilesets[0];
    // dbg!(tileset.padding);
    // dbg!(tileset.px_hei);
    // dbg!(tileset.px_wid);
    // dbg!(tileset.spacing);
    // dbg!(tileset.tile_grid_size);
    // dbg!(tileset.uid);
    // dbg!(&tileset.rel_path);
    // println!("...");

    // dbg!(data.levels);
    // dbg!(data.levels.len());
    // let level = &data.levels[0];
    // dbg!(&level.identifier);
    // dbg!(level.px_hei);
    // dbg!(level.px_wid);
    // dbg!(level.uid);
    // println!("...");

    // dbg!(level.layer_instances.len());
    // println!("...");

    // for (_index, layer) in level.layer_instances.iter().enumerate() {
    // dbg!(layer.__c_hei);
    // dbg!(layer.__c_wid);
    // dbg!(layer.__grid_size);
    // dbg!(&layer.__identifier);
    // dbg!(layer.__opacity);
    // dbg!(layer.__px_total_offset_x);
    // dbg!(layer.__px_total_offset_y);
    // dbg!(&layer.__type);
    // if let Some(ref auto_layer_tiles) = layer.auto_layer_tiles {
    //     if dbg!(auto_layer_tiles.len()) > 0 {
    //         let auto_layer_tile = &auto_layer_tiles[0];
    //         dbg!(auto_layer_tile);
    //     }
    // }
    // if let Some(ref entity_instances) = layer.entity_instances {
    //     if dbg!(entity_instances.len()) > 0 {
    //         let entity_instance = &entity_instances[0];
    //         dbg!(entity_instance);
    //     }
    // }
    // if let Some(ref grid_tiles) = layer.grid_tiles {
    //     if dbg!(grid_tiles.len()) > 0 {
    //         let grid_tile = &grid_tiles[0];
    //         dbg!(grid_tile);
    //     }
    // }
    // if let Some(ref int_grid) = layer.int_grid {
    //     if dbg!(int_grid.len()) > 0 {
    //         let int_grid_cell = &int_grid[0];
    //         dbg!(int_grid_cell);
    //     }
    // }
    // dbg!(layer.px_offset_x);
    // dbg!(layer.px_offset_y);
    // dbg!(layer.layer_def_uid);
    // println!("...");
    // }

    // let layer1 = &level.layer_instances[1];
    // dbg!(layer1.level_id);

    data.levels[0].layer_instances.iter().for_each(|layer| {
        dbg!(&layer.__type);
        dbg!(&layer.auto_layer_tiles.as_ref().unwrap().len());
        dbg!(&layer.entity_instances.as_ref().unwrap().len());
        dbg!(&layer.grid_tiles.as_ref().unwrap().len());
        dbg!(&layer.int_grid.as_ref().unwrap().len());
        println!("...");
    });

    let level = data.levels.remove(0); // TODO: more than one level `data.levels`, load on demand

    let color = &data.bg_color[1..7];
    let color = Color::hex(color).unwrap();

    App::build() // TODO: asset loader
        .add_resource(data.defs.tilesets) // TODO: lazy-load
        .add_resource(data.defs.layers) // TODO: lazy-load
        .add_resource(ClearColor(color)) // TODO: lazy-load
        .add_resource(level.layer_instances)
        .init_resource::<TilesetHandles>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(load_tileset_textures)
        .add_startup_system(setup)
        .add_system(load_atlas_textures)
        .add_system(spawn_sprites)
        .add_system(animate_sprites)
        .add_system(move_camera)
        .run();

    Ok(())
}

#[derive(Default)]
pub struct TilesetHandles {
    /// tileset uid to texture handle
    texture_handles: HashMap<i32, Handle<Texture>>,
    /// whether this is loaded yet or not
    loaded: bool,
    /// tileset uid to texture atlas handle
    atlas_handles: HashMap<i32, Handle<TextureAtlas>>,
    /// layer uid to tileset uid
    layer_to_tileset: HashMap<i32, i32>,
    /// tileset uid to position to tileset index
    position_to_index: HashMap<i32, HashMap<(i32, i32), u32>>,
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
    tilesets: Res<Vec<TilesetDefinition>>,
    layers: Res<Vec<LayerDefinition>>,
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
        // dbg!(tileset_handles.texture_handles.len());
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
            let padding = tileset.padding;
            let height = tileset.px_hei;
            let width = tileset.px_wid;
            let spacing = tileset.spacing;
            let tile = tileset.tile_grid_size;
            let uid = tileset.uid;
            let tileset_handle = tileset_handles.texture_handles.get(&uid).unwrap();
            let columns = (width - 2 * padding + spacing) / (tile + spacing);
            let rows = (height - 2 * padding + spacing) / (tile + spacing);

            // TODO: consider approach of `bevy::sprite::TextureAtlasBuilder::place_texture()`
            // i.e.
            // ```
            // atlas_texture.data[begin..end]
            //     .copy_from_slice(&texture.data[texture_begin..texture_end]);
            // ```
            // all of the below is kind of pointless, as ldtk doesn't make use of this grid anyway
            // instead we should manually create our own texture atlases
            let texture_atlas = TextureAtlas::from_grid_with_padding(
                tileset_handle.clone(),
                Vec2::splat(tile as f32),
                columns as usize,
                rows as usize,
                Vec2::splat(spacing as f32),
            );
            // dbg!(texture_atlas.textures.len());
            let texture_atlas_handle = texture_atlases.add(texture_atlas);
            tileset_handles
                .atlas_handles
                .insert(uid, texture_atlas_handle);

            let mut position_to_index = HashMap::new();
            // dbg!(rows, columns);
            for y in 0..rows {
                for x in 0..columns {
                    // dbg!(x, y);
                    // TODO: This won't work with any sort of padding
                    position_to_index.insert((x * tile, y * tile), (y * columns + x) as u32);
                }
            }
            // dbg!(position_to_index.len());
            tileset_handles
                .position_to_index
                .insert(uid, position_to_index);
        }

        for layer in layers.iter() {
            match layer.__type.as_str() {
                "AutoLayer" | "IntGrid" => {
                    if let Some(uid) = layer.auto_tileset_def_uid {
                        tileset_handles.layer_to_tileset.insert(layer.uid, uid);
                    }
                }
                "Tiles" => {
                    if let Some(uid) = layer.tileset_def_uid {
                        tileset_handles.layer_to_tileset.insert(layer.uid, uid);
                    }
                }
                "Entities" => {
                    println!("do nothing");
                }
                _ => {
                    eprintln!("unrecognised layer type");
                }
            }
        }

        tileset_handles.loaded = true;
    }
}

fn spawn_sprites(
    commands: &mut Commands,
    mut _materials: ResMut<Assets<ColorMaterial>>,
    tileset_handles: Res<TilesetHandles>,
    _tilesets: Res<Vec<TilesetDefinition>>,
    layers: Res<Vec<LayerInstance>>,
    mut done: Local<bool>,
) {
    // don't run this whole system if we've already loaded the atlas (i.e. run once only, when loaded)
    if !tileset_handles.loaded || *done {
        return;
    }
    let scale = 4.0;
    for layer in layers.iter().rev() {
        dbg!(&layer.__type);
        dbg!(&layer.layer_def_uid);
        if let Some(tileset_uid) = tileset_handles.layer_to_tileset.get(&layer.layer_def_uid) {
            let position_to_index = tileset_handles.position_to_index.get(tileset_uid).unwrap();
            match layer.__type.as_str() {
                "AutoLayer" | "IntGrid" => {
                    println!("todo: spawn actual things");
                    if let Some(ref tiles) = layer.auto_layer_tiles {
                        for tile in tiles.iter() {
                            let mut scale_x = scale;
                            let mut scale_y = scale;
                            if tile.f.x {
                                scale_x = -scale_x;
                            }
                            if tile.f.y {
                                scale_y = -scale_y;
                            }
                            let x = tile.px.x as f32 * scale;
                            let y = -(tile.px.y as f32) * scale;
                            let mut transform = Transform::from_translation(Vec3::new(x, y, 0.0));
                            transform.scale = Vec3::new(scale_x, scale_y, 0.0);

                            let index = position_to_index
                                .get(&(tile.src.x, tile.src.y))
                                .unwrap_or_else(|| panic!("src ({}, {})", tile.src.x, tile.src.y));

                            commands.spawn(SpriteSheetBundle {
                                sprite: TextureAtlasSprite::new(*index),
                                texture_atlas: tileset_handles
                                    .atlas_handles
                                    .get(tileset_uid)
                                    .unwrap()
                                    .clone(),
                                transform,
                                ..Default::default()
                            });
                        }
                    }
                }
                "Tiles" => {
                    println!("this is the simplest, so spawn these");
                    if let Some(ref tiles) = layer.grid_tiles {
                        for tile in tiles.iter() {
                            let mut scale_x = scale;
                            let mut scale_y = scale;
                            if tile.f.x {
                                scale_x = -scale_x;
                            }
                            if tile.f.y {
                                scale_y = -scale_y;
                            }
                            let x = tile.px.x as f32 * scale;
                            let y = -(tile.px.y as f32) * scale;
                            let mut transform = Transform::from_translation(Vec3::new(x, y, 0.0));
                            transform.scale = Vec3::new(scale_x, scale_y, 0.0);

                            let index = position_to_index
                                .get(&(tile.src.x, tile.src.y))
                                .unwrap_or_else(|| panic!("src ({}, {})", tile.src.x, tile.src.y));

                            commands.spawn(SpriteSheetBundle {
                                sprite: TextureAtlasSprite::new(*index),
                                texture_atlas: tileset_handles
                                    .atlas_handles
                                    .get(tileset_uid)
                                    .unwrap()
                                    .clone(),
                                transform,
                                ..Default::default()
                            });
                        }
                    }
                }
                // "IntGrid" => {
                // actually, below shouldn't technically spawn any sprites
                // but we should use it from a gameplay perspective
                // if let Some(ref int_grid) = layer.int_grid {
                //     for cell in int_grid.iter() {
                //         // dbg!(cell);
                //     }
                // }
                // }
                "Entities" => {
                    // We don't yet know what sprites we need to spawn for these entities
                    // they are too dynamic - have to handle every field type
                    // println!("todo: spawn entities");
                    // if let Some(ref entities) = layer.entity_instances {
                    //     for entity in entities.iter() {
                    //         dbg!(entity);
                    //     }
                    // }
                    println!("do nothing");
                }
                _ => {
                    eprintln!("unrecognised layer type");
                }
            }
        }
    }

    // let mut cursor = Vec2::new(640.0, 360.0);
    // const SCALE: f32 = 4.0;
    // for tileset in tilesets.iter() {
    //     let height = tileset.px_hei;
    //     let width = tileset.px_wid;
    //     let tile = tileset.tile_grid_size;
    //     let uid = tileset.uid;
    //     let size = Vec2::new(width as f32, height as f32);
    //     cursor -= size;
    //     let position = cursor + 0.5 * size;
    //     let position2_x = position.x - 0.5 * width as f32 - SCALE * tile as f32 * 0.5;
    //     let mut transform2 =
    //         Transform::from_translation(Vec2::new(position2_x, position.y).extend(0.0));
    //     transform2.scale = Vec2::new(-SCALE, SCALE).extend(0.0); // X is reversed
    //                                                              // for now, just spawn one sprite per tile
    //                                                              // set up a scene to display our texture atlas
    //     let texture_handle = tileset_handles.texture_handles.get(&uid).unwrap();
    //     // draw the atlas (tilesheet) itself
    //     commands.spawn(SpriteBundle {
    //         material: materials.add(texture_handle.clone().into()),
    //         transform: Transform::from_translation(position.extend(0.0)),
    //         ..Default::default()
    //     });
    //     // draw one sprite (tile) from the atlas (tilesheet)
    //     let texture_atlas_handle = tileset_handles.atlas_handles.get(&uid).unwrap();
    //     commands
    //         .spawn(SpriteSheetBundle {
    //             texture_atlas: texture_atlas_handle.clone(),
    //             transform: transform2,
    //             ..Default::default()
    //         })
    //         .with(Timer::from_seconds(0.5, true));
    // }

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

fn move_camera(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    const SPEED: f32 = 100.0;
    let dt = time.delta_seconds;
    let mut dx: f32 = 0.0;
    if keyboard_input.pressed(KeyCode::Left) {
        dx -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        dx += 1.0;
    }
    let mut dy: f32 = 0.0;
    if keyboard_input.pressed(KeyCode::Down) {
        dy -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Up) {
        dy += 1.0;
    }
    for mut transform in query.iter_mut() {
        if dx.abs() + dy.abs() > 0.0 {
            let v = Vec2::new(dx, dy).normalize().extend(0.0) * SPEED * dt;
            dbg!(&v);
            transform.translation += v * SPEED * dt;
        }
    }
}
