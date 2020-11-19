# ldtk-rs

Trying to integrate LDtk editor files as a bevy plugin (eventually).
So far, have just mapped the JSON structure to Rust data structures (for the most part).

## TODO

- [x] fix linting errors
- [x] finish the missing fields/types ("Objects" etc.)
- [x] add a lot of deserialisation tests (deserialise)
- [x] actually deserialise from a .ldtk file
- [ ] bevy
  - [ ] make it work
  - [ ] make it good
    - [ ] set up a custom asset + loader
    - [ ] design an API that makes use of this, i.e. map from the ldtk file to a bevy scene (entities, resources, components) and rust types
    - [ ] hot reloading
    - [ ] hot partial-reloading with diffing
- [ ] keep up with the changes to the LDtk file format

### optional

- [ ] change from arrays of ints, to structs (using serde_tuple) (partially done)
- [ ] change from structs with optional fields, to enums with unique structs (challenging)
- [ ] add a lot of serialisation tests to ensure reproducability / round-trippability
- [ ] (bevy) make it interactive the other way (changes from the bevy app write to the ldtk file)
  - note: ldtk editor doesn't hot reload though

### other ideas
- actually draw to a texture - see [SiO2](https://github.com/dmitriy-shmilo/sio2) or [`bevy::sprite::4TextureAtlasBuilder`](https://github.com/bevyengine/bevy/blob/master/crates/bevy_sprite/src/texture_atlas_builder.rs)
- construct a tilemap mesh, where its vertices UV-map many-to-many with a tileset (sprite sheet / texture atlas) texture - see [bevy_tiled] (but I think [bevy_tilemap]() abstracts this away from us)
