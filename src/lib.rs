//! # LDtk Json structure (version 0.5.1-beta)
//!
//! <https://github.com/deepnight/ldtk/blob/master/JSON_DOC.md>
//!
//! This was converted by hand, in an hour one evening,
//! from the above Markdown documentation, to Rust code,
//! using lots of regular expressions and the serde and serde_json docs.

use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

type Dynamic = Value; // temporary
type Float = f32;
type Int = i32;
type Bool = bool;
type Array<T> = Vec<T>;

#[test]
fn autolayers_1_basic() -> serde_json::Result<()> {
    let s = include_str!("../assets/AutoLayers_1_basic.ldtk");
    // dbg!(s);
    // println!("{}", s);
    // for (i, line) in s.lines().enumerate() {
    //     println!("{: <3}: {}", i + 1, line);
    // }
    let data: Root = serde_json::from_str(s)?;
    dbg!(data);
    Ok(())
}

#[test]
fn autolayers_2_stamps() -> serde_json::Result<()> {
    let s = include_str!("../assets/AutoLayers_2_stamps.ldtk");
    // dbg!(s);
    // println!("{}", s);
    // for (i, line) in s.lines().enumerate() {
    //     println!("{: <3}: {}", i + 1, line);
    // }
    let data: Root = serde_json::from_str(s)?;
    dbg!(data);
    Ok(())
}

#[test]
fn autolayers_3_mosaic() -> serde_json::Result<()> {
    let s = include_str!("../assets/AutoLayers_3_Mosaic.ldtk");
    // dbg!(s);
    // println!("{}", s);
    // for (i, line) in s.lines().enumerate() {
    //     println!("{: <3}: {}", i + 1, line);
    // }
    let data: Root = serde_json::from_str(s)?;
    dbg!(data);
    Ok(())
}

#[test]
fn autolayers_4_advanced() -> serde_json::Result<()> {
    let s = include_str!("../assets/AutoLayers_4_advanced.ldtk");
    // dbg!(s);
    // println!("{}", s);
    // for (i, line) in s.lines().enumerate() {
    //     println!("{: <3}: {}", i + 1, line);
    // }
    let data: Root = serde_json::from_str(s)?;
    dbg!(data);
    Ok(())
}

#[test]
fn entities() -> serde_json::Result<()> {
    let s = include_str!("../assets/Entities.ldtk");
    // dbg!(s);
    // println!("{}", s);
    // for (i, line) in s.lines().enumerate() {
    //     println!("{: <3}: {}", i + 1, line);
    // }
    let data: Root = serde_json::from_str(s)?;
    dbg!(data);
    Ok(())
}

#[test]
fn typical_2d_platformer_example() -> serde_json::Result<()> {
    let s = include_str!("../assets/Typical_2D_platformer_example.ldtk");
    // dbg!(s);
    // println!("{}", s);
    // for (i, line) in s.lines().enumerate() {
    //     println!("{: <3}: {}", i + 1, line);
    // }
    let data: Root = serde_json::from_str(s)?;
    dbg!(data);
    Ok(())
}

#[test]
fn typical_topdown_example() -> serde_json::Result<()> {
    let s = include_str!("../assets/Typical_TopDown_example.ldtk");
    // dbg!(s);
    // println!("{}", s);
    // for (i, line) in s.lines().enumerate() {
    //     println!("{: <3}: {}", i + 1, line);
    // }
    let data: Root = serde_json::from_str(s)?;
    dbg!(data);
    Ok(())
}

/// LDtk Json root
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Root {
    /// Project background color
    #[serde(rename = "bgColor")]
    pub bg_color: String,
    /// Default grid size for new layers
    #[serde(rename = "defaultGridSize")]
    pub default_grid_size: Int,
    /// Default X pivot (0 to 1) for new entities
    #[serde(rename = "defaultPivotX")]
    pub default_pivot_x: Float,
    /// Default Y pivot (0 to 1) for new entities
    #[serde(rename = "defaultPivotY")]
    pub default_pivot_y: Float,
    /// A structure containing all the definitions of this project
    #[serde(rename = "defs")]
    pub defs: Definitions,
    /// If TRUE, a Tiled compatible file will also be generated along with the LDtk JSON file (default is FALSE)
    #[serde(rename = "exportTiled")]
    pub export_tiled: Bool,
    /// File format version
    #[serde(rename = "jsonVersion")]
    pub json_version: String,
    /// Array of [Level]
    #[serde(rename = "levels")]
    pub levels: Array<Level>,
    /// If TRUE, the Json is partially minified (no indentation, nor line breaks, default is FALSE)
    #[serde(rename = "minifyJson")]
    pub minify_json: Bool,
}

impl Root {
    pub fn new(s: &str) -> serde_json::Result<Self> {
        serde_json::from_str(s)
    }
}

/// Level
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Level {
    /// Unique String identifier
    #[serde(rename = "identifier")]
    pub identifier: String,
    /// Array of [`LayerInstance`]
    #[serde(rename = "layerInstances")]
    pub layer_instances: Array<LayerInstance>,
    /// Height of the level in pixels
    #[serde(rename = "pxHei")]
    pub px_hei: Int,
    /// Width of the level in pixels
    #[serde(rename = "pxWid")]
    pub px_wid: Int,
    /// Unique Int identifier
    #[serde(rename = "uid")]
    pub uid: Int,
}

/// Layer instance
// TODO: Consider splitting up into enum LayerInstance { IntGrid(IntGridLayerInstance), Entity(EntityLayerInstance), Auto(AutoLayerInstance), Tile(TileLayerInstance) } with #[serde(tag = "__type")] (internally tagged representation) instead of using Option<T>'s
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LayerInstance {
    /// Layer type (possible values: IntGrid, Entities, Tiles or AutoLayer)
    // TODO: see above TODO
    #[serde(rename = "__type")]
    pub __type: String,
    /// Grid-based height
    #[serde(rename = "__cHei")]
    pub __c_hei: Int,
    /// Grid-based width
    #[serde(rename = "__cWid")]
    pub __c_wid: Int,
    /// Grid size
    #[serde(rename = "__gridSize")]
    pub __grid_size: Int,
    /// Unique String identifier
    #[serde(rename = "__identifier")]
    pub __identifier: String,
    /// (Added 0.4.0) Layer opacity as Float [0-1]
    #[serde(rename = "__opacity")]
    pub __opacity: Float,
    /// (Added 0.5.0) Total layer X pixel offset, including both instance and definition offsets.
    #[serde(rename = "__pxTotalOffsetX")]
    pub __px_total_offset_x: Int,
    /// (Added 0.5.0) Total layer Y pixel offset, including both instance and definition offsets.
    #[serde(rename = "__pxTotalOffsetY")]
    pub __px_total_offset_y: Int,
    /// (Added 0.4.0) <sup>Only *Auto-layers*</sup> An array containing all tiles generated by Auto-layer rules. The array is already sorted in display order (ie. 1st tile is beneath 2nd, which is beneath 3rd etc.).          Note: if multiple tiles are stacked in the same cell as the result of different rules, all tiles behind opaque ones will be discarded.
    // TODO: see above TODO: unwrap from Option<T> when included, omit when not included
    #[serde(rename = "autoLayerTiles")]
    pub auto_layer_tiles: Option<Array<TileInstance>>,
    /// <sup>Only *Entity layers*</sup>
    // TODO: see above TODO: unwrap from Option<T> when included, omit when not included
    #[serde(rename = "entityInstances")]
    pub entity_instances: Option<Array<EntityInstance>>,
    /// <sup>Only *Tile layers*</sup>
    // TODO: see above TODO: unwrap from Option<T> when included, omit when not included
    #[serde(rename = "gridTiles")]
    pub grid_tiles: Option<Array<TileInstance>>,
    /// <sup>Only *IntGrid layers*</sup>
    // TODO: see above TODO: unwrap from Option<T> when included, omit when not included
    #[serde(rename = "intGrid")]
    pub int_grid: Option<Array<IntGridCell>>,
    /// Reference the Layer definition UID
    #[serde(rename = "layerDefUid")]
    pub layer_def_uid: Int,
    /// Reference to the UID of the level containing this layer instance
    #[serde(rename = "levelId")]
    pub level_id: Int,
    /// (Changed 0.5.0) X offset in pixels to render this layer, usually 0 (IMPORTANT: this should be added to the `LayerDef` optional offset)
    #[serde(rename = "pxOffsetX")]
    pub px_offset_x: Int,
    /// (Changed 0.5.0) Y offset in pixels to render this layer, usually 0 (IMPORTANT: this should be added to the `LayerDef` optional offset)
    #[serde(rename = "pxOffsetY")]
    pub px_offset_y: Int,
    /// <sup>Only *Auto-layers*</sup> Random seed used for Auto-Layers rendering
    // TODO: see above TO: unwrap from Option<T>DO
    #[serde(rename = "seed")]
    pub seed: Option<Int>,
}
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IntGridCell {
    /// Coordinate ID in the layer grid
    #[serde(rename = "coordId")]
    pub coord_id: Int,
    /// IntGrid value
    #[serde(rename = "v")]
    pub v: Int,
}

/// Tile instance (Added 0.4.0)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TileInstance {
    /// Internal data used by the editor.
    /// For auto-layer tiles: `[ruleId, coordId, tileId]`.
    /// For tile-layer tiles: `[coordId, tileId]`.
    // TODO: Deserialize as a struct AutoLayerTileIds { ruleId, coordId, tileId } and struct TileLayerTileIds { coordId, tileId } instead using serde_tuple
    #[serde(rename = "d")]
    pub d: Array<Int>,
    /// "Flip bits", a 2-bits integer to represent the mirror transformations of the tile.
    ///
    /// - Bit 0 = X flip
    /// - Bit 1 = Y flip
    ///
    /// Examples:
    ///
    /// - f=0 (no flip)
    /// - f=1 (X flip only)
    /// - f=2 (Y flip only)
    /// - f=3 (both flips)
    #[serde(rename = "f")]
    pub f: Flip,
    /// (Changed 0.5.0) Pixel coordinates of the tile in the **layer** (`[x,y]` format). Don't forget optional layer offsets, if they exist!
    // TODO: Deserialize as a struct Coord { x, y } instead using serde_tuple
    #[serde(rename = "px")]
    pub px: Coord,
    /// Pixel coordinates of the tile in the **tileset** (`[x,y]` format)
    // TODO: Deserialize as a struct Coord { x, y } instead using serde_tuple
    #[serde(rename = "src")]
    pub src: Coord,
}

/// Entity instance
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntityInstance {
    /// (Changed 0.4.0) Grid-based coordinates (`[x,y]` format)
    // TODO: Deserialize as a struct Coord { x, y }  instead using serde_tuple
    #[serde(rename = "__grid")]
    pub __grid: Coord,
    /// Unique String identifier
    #[serde(rename = "__identifier")]
    pub __identifier: String,
    /// (Can be `null`/`None`) (Added 0.4.0) Optional Tile used to display this entity (it could either be the default Entity tile, or some tile provided by a field value, like an Enum).
    #[serde(rename = "__tile")]
    pub __tile: Option<EntityTile>,
    /// Reference of the **Entity definition** UID
    #[serde(rename = "defUid")]
    pub def_uid: Int,
    ///
    #[serde(rename = "fieldInstances")]
    pub field_instances: Array<FieldInstance>,
    /// (Changed 0.4.0) Pixel coordinates (`[x,y]` format). Don't forget optional layer offsets, if they exist!
    // TODO: Deserialize as a struct Coord { x, y }  instead using serde_tuple
    #[serde(rename = "px")]
    pub px: Coord,
}

///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntityTile {
    /// An array of 4 Int values that refers to the tile in the tileset image: `[ x, y, width, height ]`
    // TODO: change to struct Rect { x, y, width, height }
    #[serde(rename = "srcRect")]
    pub src_rect: Rect,
    /// Tileset ID
    #[serde(rename = "tilesetUid")]
    pub tileset_uid: Int,
}

/// Field instance
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FieldInstance {
    /// Unique String identifier
    #[serde(rename = "__identifier")]
    pub __identifier: String,
    /// Type of the field, such as Int, Float, Enum(enum_name), Bool, etc.
    #[serde(rename = "__type")]
    pub __type: String,
    /// (Anything) Actual value of the field instance. The value type may vary, depending on `__type` (Integer, Boolean, String etc.)
    /// It can also be an `Array` of various types.
    #[serde(rename = "__value")]
    pub __value: Dynamic,
    /// Reference of the **Field definition** UID
    #[serde(rename = "defUid")]
    pub def_uid: Int,
}

/// Definitions
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Definitions {
    ///
    #[serde(rename = "entities")]
    pub entities: Array<EntityDefinition>,
    ///
    #[serde(rename = "enums")]
    pub enums: Array<EnumDefinition>,
    /// Note: external enums are exactly the same as `enums`, except they
    /// have a `relPath` to point to an external source file.
    #[serde(rename = "externalEnums")]
    pub external_enums: Array<EnumDefinition>,
    ///
    #[serde(rename = "layers")]
    pub layers: Array<LayerDefinition>,
    ///
    #[serde(rename = "tilesets")]
    pub tilesets: Array<TilesetDefinition>,
}

/// Layer definition
// TODO: Consider splitting up into enum LayerDefinition { IntGrid(IntGridLayerDefinition), Entity(EntityLayerDefinition), Auto(AutoLayerDefinition), Tile(TileLayerDefinition) } with #[serde(tag = "__type")] (internally tagged representation) instead of using Option<T>'s
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LayerDefinition {
    /// Type of the layer (*IntGrid, Entities, Tiles or AutoLayer*)
    // TODO: see above TODO
    #[serde(rename = "__type")]
    pub __type: String,
    /// <sup>Only *Auto-layers*</sup> Contains all the auto-layer rule definitions.
    // TODO: see above TODO: unwrap from Option<T> when included, omit when not included
    #[serde(rename = "autoRuleGroups")]
    pub auto_rule_groups: Option<Array<AutoRuleGroup>>,
    /// <sup>Only *Auto-layers*</sup>
    // TODO: see above TODO: unwrap from Option<T> when included, omit when not included
    #[serde(rename = "autoSourceLayerDefUid")]
    pub auto_source_layer_def_uid: Option<Int>,
    /// <sup>Only *Auto-layers* and *IntGrid layers*</sup> Reference to the Tileset UID being used by this auto-layer rules
    // TODO: see above TODO: unwrap from Option<T> when included, omit when not included
    #[serde(rename = "autoTilesetDefUid")]
    pub auto_tileset_def_uid: Option<Int>,
    /// Opacity of the layer (0 to 1.0)
    #[serde(rename = "displayOpacity")]
    pub display_opacity: Float,
    /// Width and height of the grid in pixels
    #[serde(rename = "gridSize")]
    pub grid_size: Int,
    /// Unique String identifier
    #[serde(rename = "identifier")]
    pub identifier: String,
    /// <sup>Only *IntGrid layer*</sup>
    // TODO: see above TODO: unwrap from Option<T> when included, omit when not included
    #[serde(rename = "intGridValues")]
    pub int_grid_values: Option<Array<IntGridValue>>,
    /// (Added 0.5.0) X offset of the layer, in pixels (IMPORTANT: this should be added to the `LayerInstance` optional offset)
    #[serde(rename = "pxOffsetX")]
    pub px_offset_x: Int,
    /// (Added 0.5.0) Y offset of the layer, in pixels (IMPORTANT: this should be added to the `LayerInstance` optional offset)
    #[serde(rename = "pxOffsetY")]
    pub px_offset_y: Int,
    /// <sup>Only *Tile layers*</sup> If the tiles are smaller or larger than the layer grid, the pivot value will be used to position the tile relatively its grid cell.
    // TODO: see above TODO: unwrap from Option<T> when included, omit when not included
    #[serde(rename = "tilePivotX")]
    pub tile_pivot_x: Option<Float>,
    /// <sup>Only *Tile layers*</sup> If the tiles are smaller or larger than the layer grid, the pivot value will be used to position the tile relatively its grid cell.
    // TODO: see above TODO: unwrap from Option<T> when included, omit when not included
    #[serde(rename = "tilePivotY")]
    pub tile_pivot_y: Option<Float>,
    /// <sup>Only *Tile layers*</sup> Reference to the Tileset UID being used by this tile layer
    // TODO: see above TODO: unwrap from Option<T> when included, omit when not included
    #[serde(rename = "tilesetDefUid")]
    pub tileset_def_uid: Option<Int>,
    /// Unique Int identifier
    #[serde(rename = "uid")]
    pub uid: Int,
}

/// IntGrid value
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IntGridValue {
    /// Hex color "#rrggbb"
    color: String,
    ///
    identifier: Option<String>,
}

///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutoRuleGroup {
    ///
    #[serde(rename = "active")]
    pub active: Bool,
    ///
    #[serde(rename = "collapsed")]
    pub collapsed: Bool,
    ///
    #[serde(rename = "name")]
    pub name: String,
    ///
    #[serde(rename = "rules")]
    pub rules: Array<AutoLayerRuleDefinition>,
    ///
    #[serde(rename = "uid")]
    pub uid: Int,
}

/// Auto-layer rule definition
// TODO: Consider splitting up into enum AutoLayerRuleDefinition { Stamp(StampRuleDefinition), Single(SingleRuleDefinition) } with #[serde(tag = "tileMode")] (internally tagged representation) instead of using Option<T>'s
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutoLayerRuleDefinition {
    /// Defines how tileIds array is used, either "Stamp" or "Single"
    // TODO: see above TODO
    #[serde(rename = "tileMode")]
    pub tile_mode: String,
    /// If FALSE, the rule effect isn't applied, and no tiles are generated.
    #[serde(rename = "active")]
    pub active: Bool,
    /// When TRUE, the rule will prevent other rules to be applied in the same cell if it matches (TRUE by default).
    #[serde(rename = "breakOnMatch")]
    pub break_on_match: Bool,
    /// Chances for this rule to be applied (0 to 1)
    #[serde(rename = "chance")]
    pub chance: Float,
    /// If TRUE, enable checker mode
    #[serde(rename = "checker")]
    pub checker: Checker,
    /// If TRUE, allow rule to be matched by flipping its pattern horizontally
    #[serde(rename = "flipX")]
    pub flip_x: Bool,
    /// If TRUE, allow rule to be matched by flipping its pattern vertically
    #[serde(rename = "flipY")]
    pub flip_y: Bool,
    /// Rule pattern (size x size)
    ///
    /// e.g. For an IntGrid with 2 grid values (1 = "walls" and 2 = "floor")
    /// for a tile size of 3, there are (3x3) 9 cells:
    /// `[0, 0, 0, 2, 1, 2, 0, 0, 0]` is equivalent to
    /// ```compile_fail
    /// 000
    /// 212
    /// 000
    /// ```
    /// i.e. at the centre position, if it is a wall (grid value = 1),
    /// where there is floor (2) either side of it
    /// and anything (0) above and below it, then apply the rule.
    ///
    /// This can also be negative: -2 means if this grid value (floor) is present,
    /// **don't** draw the cell.
    #[serde(rename = "pattern")]
    pub pattern: Array<Int>,
    /// If TRUE, enable Perlin filtering to only apply rule on specific random area
    #[serde(rename = "perlinActive")]
    pub perlin_active: Bool,
    ///
    #[serde(rename = "perlinOctaves")]
    pub perlin_octaves: Float,
    ///
    #[serde(rename = "perlinScale")]
    pub perlin_scale: Float,
    ///
    #[serde(rename = "perlinSeed")]
    pub perlin_seed: Float,
    /// <sup>Only *'Stamp' tile mode*</sup> X pivot of a tile stamp (0-1)
    // TODO: see above TODO: unwrap from Option<T> when included, omit when not included
    #[serde(rename = "pivotX")]
    pub pivot_x: Option<Float>,
    /// <sup>Only *'Stamp' tile mode*</sup> Y pivot of a tile stamp (0-1)
    // TODO: see above TODO: unwrap from Option<T> when included, omit when not included
    #[serde(rename = "pivotY")]
    pub pivot_y: Option<Float>,
    /// Pattern width & height. Should only be 1,3,5 or 7.
    #[serde(rename = "size")]
    pub size: Int,
    /// Array of all the tile IDs. They are used randomly or as stamps, based on `tileMode` value.
    #[serde(rename = "tileIds")]
    pub tile_ids: Array<Int>,
    /// Unique Int identifier
    #[serde(rename = "uid")]
    pub uid: Int,
    /// X cell coord modulo
    #[serde(rename = "xModulo")]
    pub x_modulo: Int,
    /// Y cell coord modulo
    #[serde(rename = "yModulo")]
    pub y_modulo: Int,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Checker {
    Horizontal,
    None,
}

/// Entity definition
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntityDefinition {
    /// Base entity color
    #[serde(rename = "color")]
    pub color: String,
    /// Array of field definitions
    #[serde(rename = "fieldDefs")]
    pub field_defs: Array<FieldDefinition>,
    /// Pixel height
    #[serde(rename = "height")]
    pub height: Int,
    /// Unique String identifier
    #[serde(rename = "identifier")]
    pub identifier: String,
    /// Max instances per level
    #[serde(rename = "maxPerLevel")]
    pub max_per_level: Int,
    /// Pivot X coordinate (from 0 to 1.0)
    #[serde(rename = "pivotX")]
    pub pivot_x: Float,
    /// Pivot Y coordinate (from 0 to 1.0)
    #[serde(rename = "pivotY")]
    pub pivot_y: Float,
    /// Tile ID used for optional tile display
    #[serde(rename = "tileId")]
    pub tile_id: Option<Int>,
    /// Tileset ID used for optional tile display
    #[serde(rename = "tilesetId")]
    pub tileset_id: Option<Int>,
    /// Unique Int identifier
    #[serde(rename = "uid")]
    pub uid: Int,
    /// Pixel width
    #[serde(rename = "width")]
    pub width: Int,
}

/// Field definition
///
/// Sorry this type has no documentation yet.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FieldDefinition {
    ///
    #[serde(rename = "__type")]
    pub __type: String,
    ///
    #[serde(rename = "identifier")]
    pub identifier: String,
    ///
    #[serde(rename = "uid")]
    pub uid: Int,
    ///
    // appears to be used internally, can sometimes be an object similar to FieldDefaultValue
    #[serde(rename = "type")]
    pub field_type: FieldType,
    ///
    #[serde(rename = "isArray")]
    pub is_array: Bool,
    ///
    #[serde(rename = "canBeNull")]
    pub can_be_null: Bool,
    ///
    #[serde(rename = "arrayMinLength")]
    pub array_min_length: Option<Int>,
    ///
    #[serde(rename = "arrayMaxLength")]
    pub array_max_length: Option<Int>,
    ///
    #[serde(rename = "editorDisplayMode")]
    pub editor_display_mode: String,
    ///
    #[serde(rename = "editorDisplayPos")]
    pub editor_display_pos: String,
    ///
    #[serde(rename = "editorAlwaysShow")]
    pub editor_always_show: Bool,
    ///
    #[serde(rename = "min")]
    pub min: Option<Int>,
    ///
    #[serde(rename = "max")]
    pub max: Option<Int>,
    ///
    #[serde(rename = "FieldDefaultValue")]
    pub default_override: Option<FieldDefaultValue>,
}

/// Field type
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
// this should hopefuly deserialize OK, but might be wrong when it re-serializes
pub enum FieldType {
    Simple(String),
    Complex(FieldComplexType),
}

/// Complex field type
///
/// When pointing to an Enum, the param is the uid of the Enum
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FieldComplexType {
    ///
    #[serde(rename = "id")]
    pub id: String,
    ///
    #[serde(rename = "params")]
    pub params: Array<Param>,
}

/// Specifies the default for a field
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FieldDefaultValue {
    ///
    #[serde(rename = "id")]
    pub id: String,
    ///
    #[serde(rename = "params")]
    pub params: Array<Param>,
}

/// Param
// this should hopefuly deserialize OK, but might be wrong when it re-serializes
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Param {
    String(String),
    Int(Int),
}

/// Tileset definition
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TilesetDefinition {
    /// Unique String identifier
    #[serde(rename = "identifier")]
    pub identifier: String,
    /// (Can be `null`/`None`) (Added 0.5.0) An array of all tiles that are fully opaque (ie. no transparent pixel). Used internally for optimizations.
    #[serde(rename = "opaqueTiles")]
    pub opaque_tiles: Option<Array<Int>>,
    /// Distance in pixels from image borders
    #[serde(rename = "padding")]
    pub padding: Int,
    /// Image width in pixels
    #[serde(rename = "pxHei")]
    pub px_hei: Int,
    /// Image width in pixels
    #[serde(rename = "pxWid")]
    pub px_wid: Int,
    /// Path to the source file, relative to the current project JSON file
    #[serde(rename = "relPath")]
    pub rel_path: String,
    /// Space in pixels between all tiles
    #[serde(rename = "spacing")]
    pub spacing: Int,
    ///
    #[serde(rename = "tileGridSize")]
    pub tile_grid_size: Int,
    /// Unique Identifier
    #[serde(rename = "uid")]
    pub uid: Int,
}

/// Enum definition
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnumDefinition {
    /// (Can be `null`/`None`) Relative path to the external file providing this Enum
    #[serde(rename = "externalRelPath")]
    pub external_rel_path: Option<String>,
    /// (Can be `null`/`None`) Tileset UID if provided
    #[serde(rename = "iconTilesetUid")]
    pub icon_tileset_uid: Option<Int>,
    /// Unique String identifier
    #[serde(rename = "identifier")]
    pub identifier: String,
    /// Unique Int identifier
    #[serde(rename = "uid")]
    pub uid: Int,
    /// All possible enum values, with their optional Tile infos.
    #[serde(rename = "values")]
    pub values: Array<EnumValue>,
}

///
// TODO: provide a way to map this from a LDtk representation to a first-class Rust representation
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnumValue {
    /// (Added 0.4.0) An array of 4 Int values that refers to the tile in the tileset image: `[ x, y, width, height ]`
    // TODO: Deserialize as a struct Rect { x, y, width, height } instead using serde_tuple
    #[serde(rename = "__tileSrcRect")]
    pub __tile_src_rect: Rect,
    /// Enum value
    #[serde(rename = "id")]
    pub id: String,
    /// (Can be `null`/`None`) The optional ID of the tile
    #[serde(rename = "tileId")]
    pub tile_id: Option<Int>,
}

///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rect {
    pub x: Int,
    pub y: Int,
    pub width: Int,
    pub height: Int,
}

///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Coord {
    pub x: Int,
    pub y: Int,
}

///
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Flip {
    pub x: bool,
    pub y: bool,
}

impl<'de> Deserialize<'de> for Flip {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let i: i32 = Deserialize::deserialize(deserializer)?;
        match i {
            0 => Ok(Self { x: false, y: false }),
            1 => Ok(Self { x: true, y: false }),
            2 => Ok(Self { x: false, y: true }),
            3 => Ok(Self { x: true, y: true }),
            _ => Err(D::Error::custom(format!("invalid value for flip: {}", i))),
        }
    }
}

impl Serialize for Flip {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut i = 0;
        if self.x {
            i += 1;
        }
        if self.y {
            i += 2;
        }
        serializer.serialize_i32(i)
    }
}

#[test]
fn test_flip() -> serde_json::Result<()> {
    let s = "0";
    let flip: Flip = serde_json::from_str(s)?;
    assert_eq!(flip, Flip { x: false, y: false });
    let s_roundtrip = serde_json::to_string(&flip)?;
    assert_eq!(s, s_roundtrip);

    let s = "1";
    let flip: Flip = serde_json::from_str(s)?;
    assert_eq!(flip, Flip { x: true, y: false });
    let s_roundtrip = serde_json::to_string(&flip)?;
    assert_eq!(s, s_roundtrip);

    let s = "2";
    let flip: Flip = serde_json::from_str(s)?;
    assert_eq!(flip, Flip { x: false, y: true });
    let s_roundtrip = serde_json::to_string(&flip)?;
    assert_eq!(s, s_roundtrip);

    let s = "3";
    let flip: Flip = serde_json::from_str(s)?;
    assert_eq!(flip, Flip { x: true, y: true });
    let s_roundtrip = serde_json::to_string(&flip)?;
    assert_eq!(s, s_roundtrip);

    // TODO: test bad data
    // let s = "4";
    // let flip: Flip = serde_json::from_str(s)?;
    // assert_eq!(flip, Flip { x: true, y: true });
    // let s_roundtrip = serde_json::to_string(&flip)?;
    // assert_eq!(s, s_roundtrip);

    Ok(())
}
