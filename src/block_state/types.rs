#[derive(Debug, strum::Display, strum::EnumString, Clone, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum Direction { North, East, South, West, Up, Down }
#[derive(Debug, strum::Display, strum::EnumString, Clone, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum VerticalDirection { Up, Down }
#[derive(Debug, strum::Display, strum::EnumString, Clone, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum HorizontalDirection { North, East, South, West }
#[derive(Debug, strum::Display, strum::EnumString, Clone, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum HopperDirection { North, East, South, West, Down }
#[derive(Debug, strum::Display, strum::EnumString, Clone, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum Axis { X, Y, Z }
#[derive(Debug, strum::Display, strum::EnumString, Clone, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum HorizontalAxis { X, Z }
#[derive(Debug, strum::Display, strum::EnumString, Clone, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum BambooLeaves { None, Small, Large }
#[derive(Debug, strum::Display, strum::EnumString, Clone, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum WallShape { None, Low, Tail }
#[derive(Debug, strum::Display, strum::EnumString, Clone, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum Tilt { None, Unstable, Partial, Full }
#[derive(Debug, strum::Display, strum::EnumString, Clone, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum WireConnection { None, Up, Side }
#[derive(Debug, strum::Display, strum::EnumString, Clone, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum Instrument {
    Harp,
    Basedrum,
    Snare,
    Hat,
    Bass,
    Flute,
    Bell,
    Guitar,
    Chime,
    Xylophone,
    IronXylophone,
    CowBell,
    Didgeridoo,
    Bit,
    Banjo,
    Pling,
}
#[derive(Debug, strum::Display, strum::EnumString, Clone, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum BedPart { Head, Foot }
#[derive(Debug, strum::Display, strum::EnumString, Clone, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum RailShape {
    NorthSouth,
    EastWest,
    AscendingEast,
    AscendingWest,
    AscendingNorth,
    AscendingSouth,
    SouthEast,
    SouthWest,
    NorthWest,
    NorthEast,
}
#[derive(Debug, strum::Display, strum::EnumString, Clone, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum DoubleBlockHalf { Upper, Lower }
#[derive(Debug, strum::Display, strum::EnumString, Clone, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum BlockHalf { Top, Bottom }
#[derive(Debug, strum::Display, strum::EnumString, Clone, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum SlabType { Top, Bottom, Double }
#[derive(Debug, strum::Display, strum::EnumString, Clone, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum PistonType { Normal, Sticky }
#[derive(Debug, strum::Display, strum::EnumString, Clone, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum StairShape { Straight, InnerLeft, InnerRight, OuterLeft, OuterRight }
#[derive(Debug, strum::Display, strum::EnumString, Clone, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum ChestType { Single, Left, Right }
#[derive(Debug, strum::Display, strum::EnumString, Clone, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum DoorHinge { Left, Right }
#[derive(Debug, strum::Display, strum::EnumString, Clone, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum WallMountLocation { Floor, Wall, Ceiling }
#[derive(Debug, strum::Display, strum::EnumString, Clone, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum Attachment { Floor, SingleWall, DoubleWall, Ceiling }
#[derive(Debug, strum::Display, strum::EnumString, Clone, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum ComparatorMode { Compare, Subtract }
#[derive(Debug, strum::Display, strum::EnumString, Clone, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum StructureBlockMode { Save, Load, Corner, Data }
#[derive(Debug, strum::Display, strum::EnumString, Clone, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum Orientation {
    DownEast,
    DownNorth,
    DownSouth,
    DownWest,
    UpEast,
    UpNorth,
    UpSouth,
    UpWest,
    WestUp,
    EastUp,
    NorthUp,
    SouthUp,
}
#[derive(Debug, strum::Display, strum::EnumString, Clone, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum SculkSensorPhase { Inactive, Active, Cooldown }
#[derive(Debug, strum::Display, strum::EnumString, Clone, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum Thickness { TipMerge, Tip, Frustum, Middle, Base }
