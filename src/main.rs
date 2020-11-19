use bevy::{asset::LoadState, prelude::*, sprite::TextureAtlasBuilder};
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
    let json = include_str!(files!(TYPICAL_2D_PLATFORMER_EXAMPLE));
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

    for (index, layer) in level.layer_instances.iter().enumerate() {
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
        .add_startup_system(load_tilesets)
        .add_system(load_atlas)
        .add_system(animate_sprite_system)
        .run();

    Ok(())
}

#[derive(Default)]
pub struct TilesetHandles {
    handles: HashMap<i32, Handle<Texture>>,
    atlas_loaded: bool,
}

fn load_tilesets(
    commands: &mut Commands,
    tilesets: Res<Vec<TilesetDefinition>>,
    mut tileset_handles: ResMut<TilesetHandles>,
    asset_server: Res<AssetServer>,
) {
    // load the folder and create handles to the files
    for tileset in tilesets.iter() {
        tileset_handles.handles.insert(
            tileset.uid,
            asset_server.load::<_, _>(tileset.rel_path.as_str()),
        );
    }
    commands.spawn(Camera2dBundle::default());
}

fn load_atlas(
    commands: &mut Commands,
    mut tileset_handles: ResMut<TilesetHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    textures: Res<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    tilesets: Res<Vec<TilesetDefinition>>,
) {
    // don't run this whole system if we've already loaded the atlas (i.e. run once only, when loaded)
    if tileset_handles.atlas_loaded {
        return;
    }

    // but if we haven't...
    // wait until texture assets are loaded
    if let LoadState::Loaded = asset_server
        .get_group_load_state(tileset_handles.handles.iter().map(|(_, handle)| handle.id))
    {
        // initialise a texture atlas builder
        // let mut texture_atlas_builder = TextureAtlasBuilder::default(); // <- why does this need to be outside though?

        // add the loaded texture assets to the builder
        // for (_, handle) in tileset_handles.handles.iter() {
        //     let texture = textures.get(handle).unwrap();
        //     texture_atlas_builder.add_texture(handle.clone_weak(), &texture);
        //     // <- what is clone_weak()?
        // }

        // build the texture atlas
        // let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();

        // // get a handle to a specific* texture
        // let vendor_handle =
        //     asset_server.get_handle("textures/rpg/chars/vendor/generic-rpg-vendor.png");

        // // before moving the new texture's handle into the atlas, get the index of that specific* texture in it
        // let vendor_index = texture_atlas.get_texture_index(&vendor_handle).unwrap();

        // before moving the new texture's handle into the atlas, clone it
        // let texture = texture_atlas.texture.clone();

        // move the new texture's handle into the texture atlas, adding it, and get the texture atlas handle
        // let atlas_handle = texture_atlases.add(texture_atlas);

        // let texture = tileset_handles.handles.iter().next().unwrap().1.clone();
        // padding left | sprite | spacing | sprite | padding right
        // p | s | x | s | p
        // n = 2
        // inner height/width = height/width - 2* padding;
        // sprite(n) + spacing(n-1) = inner height/width
        // sprite.n + spacing.n - spacing = inner height/width
        // sprite.n + spacing.n = inner height/width + spacing
        // n.(sprite + spacing) = inner height/width + spacing
        // n = (inner height/width + spacing) / (sprite + spacing)
        // n = (height/width - 2 * padding + spacing) / (sprite + spacing)
        dbg!(tileset_handles.handles.len());
        let mut cursor = Vec2::zero();
        for tileset in tilesets.iter() {
            let padding = dbg!(tileset.padding);
            let height = dbg!(tileset.px_hei);
            let width = dbg!(tileset.px_wid);
            let spacing = dbg!(tileset.spacing);
            let sprite = dbg!(tileset.tile_grid_size);
            let uid = dbg!(tileset.uid);
            let tileset_handle = tileset_handles.handles.get(&uid).unwrap();
            let columns = dbg!((width - 2 * padding + spacing) / (sprite + spacing));
            let rows = dbg!((height - 2 * padding + spacing) / (sprite + spacing));

            let texture_atlas = TextureAtlas::from_grid_with_padding(
                tileset_handle.clone(),
                Vec2::splat(sprite as f32),
                columns as usize,
                rows as usize,
                Vec2::splat(spacing as f32),
            );
            let texture_atlas_handle = texture_atlases.add(texture_atlas);
            let texture = textures.get(tileset_handle).unwrap();
            cursor -= texture.size;
            let position = cursor + 0.5 * texture.size;
            let position2_x = position.x + 0.5 * texture.size.x + 20.0 * sprite as f32 * 0.5;
            let mut transform2 =
                Transform::from_translation(Vec2::new(position2_x, position.y).extend(0.0));
            transform2.scale = Vec2::splat(20.0).extend(0.0);
            // set up a scene to display our texture atlas
            commands
                // // draw a specific* sprite from the atlas
                // .spawn(SpriteSheetBundle {
                //     transform: Transform {
                //         translation: Vec3::new(150.0, 0.0, 0.0),
                //         // scale it up 4x
                //         scale: Vec2::splat(4.0).extend(0.0),
                //         ..Default::default()
                //     },
                //     // point to the texture atlas, and get the texture atlas handle
                //     texture_atlas: atlas_handle,
                //     // take a sprite from the texture atlas
                //     sprite: TextureAtlasSprite::new(vendor_index as u32),
                //     //
                //     ..Default::default()
                // })
                // draw the atlas itself
                .spawn(SpriteBundle {
                    material: materials.add(tileset_handle.clone().into()),
                    transform: Transform::from_translation(position.extend(0.0)),
                    ..Default::default()
                })
                .spawn(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    transform: transform2,
                    ..Default::default()
                })
                .with(Timer::from_seconds(0.5, true));
        }

        tileset_handles.atlas_loaded = true;
    }
}

fn animate_sprite_system(
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
