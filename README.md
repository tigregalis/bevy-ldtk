# ldtk-rs

Trying to integrate LDtk editor files as a bevy plugin (eventually).
So far, have just mapped the JSON structure to Rust data structures (for the most part).

## TODO

- [] fix linting errors
- [] finish the missing fields/types ("Objects")
- [] actually serialise from a .ldtk file
- [] (bevy) set up a custom asset + loader
- [] (bevy) design an API that makes use of this, i.e. map from the ldtk file to a bevy scene (entities, resources, components)
- [] (bevy) make it interactive
- [] (bevy) hot reloading
- [] (bevy) hot partial-reloading with diffing
- [] keep up with the changes to the LDtk file format
