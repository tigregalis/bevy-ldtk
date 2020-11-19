use bevy::{asset::LoadState, prelude::*, sprite::TextureAtlasBuilder};
use ldtk_rs::{Root, TilesetDefinition};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json = include_str!("../assets/AutoLayers_1_basic.ldtk");
    let data = Root::new(json)?;

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

    App::build()
        .add_resource(data.defs.tilesets)
        .init_resource::<TilesetHandles>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(load_atlas)
        .run();

    Ok(())
}

#[derive(Default)]
pub struct TilesetHandles {
    handles: HashMap<i32, Handle<Texture>>,
    atlas_loaded: bool,
}

fn setup(
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
}

fn load_atlas(
    commands: &mut Commands,
    mut tileset_handles: ResMut<TilesetHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
        let mut texture_atlas_builder = TextureAtlasBuilder::default(); // <- why does this need to be outside though?

        // add the loaded texture assets to the builder
        for (_, handle) in tileset_handles.handles.iter() {
            let texture = textures.get(handle).unwrap();
            texture_atlas_builder.add_texture(handle.clone_weak(), &texture);
            // <- what is clone_weak()?
        }

        // build the texture atlas
        let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();

        // // get a handle to a specific* texture
        // let vendor_handle =
        //     asset_server.get_handle("textures/rpg/chars/vendor/generic-rpg-vendor.png");

        // // before moving the new texture's handle into the atlas, get the index of that specific* texture in it
        // let vendor_index = texture_atlas.get_texture_index(&vendor_handle).unwrap();

        // before moving the new texture's handle into the atlas, clone it
        let texture_atlas_texture = texture_atlas.texture.clone();

        // move the new texture's handle into the texture atlas, adding it, and get the texture atlas handle
        let atlas_handle = texture_atlases.add(texture_atlas);

        // set up a scene to display our texture atlas
        commands
            .spawn(Camera2dBundle::default())
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
                material: materials.add(texture_atlas_texture.into()),
                transform: Transform::from_translation(Vec3::new(-300.0, 0.0, 0.0)),
                ..Default::default()
            });

        tileset_handles.atlas_loaded = true;
    }
}
