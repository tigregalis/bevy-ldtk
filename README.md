# ldtk-rs

Trying to integrate LDtk editor files as a bevy plugin (eventually).
So far, have just mapped the JSON structure to Rust data structures (for the most part).

## TODO

- [x] fix linting errors
- [x] finish the missing fields/types ("Objects" etc.)
- [x] add a lot of deserialisation tests (deserialise)
- [x] actually deserialise from a .ldtk file
- [ ] (bevy) set up a custom asset + loader
- [ ] (bevy) design an API that makes use of this, i.e. map from the ldtk file to a bevy scene (entities, resources, components) and rust types
- [ ] (bevy) hot reloading
- [ ] (bevy) hot partial-reloading with diffing
- [ ] (bevy) make it interactive the other way
- [ ] keep up with the changes to the LDtk file format

### optional

- [ ] change from arrays of ints, to structs (using serde_tuple) (partially done)
- [ ] change from structs with optional fields, to enums with unique structs (challenging)
- [ ] add a lot of serialisation tests to ensure reproducability / round-trippability
