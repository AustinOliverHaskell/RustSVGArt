pub const LINE_RELATIVE: char = 'l';
pub const LINE_ABSOLUTE: char = 'L';
pub const VERTICAL_ABSOLUTE: char = 'V';
pub const VERTICAL_RELATIVE: char = 'v';
pub const HORIZONTAL_ABSOLUTE: char = 'H';
pub const HORIZONTAL_RELATIVE: char = 'h';
pub const CUBIC_BEZIER_ABSOLUTE: char = 'C';
pub const CUBIC_BEZIER_RELATIVE: char = 'c';
pub const SHORTHAND_CUBIC_BEZIER_ABSOLUTE: char = 'S';
pub const SHORTHAND_CUBIC_BEZIER_RELATIVE: char = 's';
pub const QUADRATIC_BEZIER_ABSOLUTE: char = 'Q';
pub const QUADRATIC_BEZIER_RELATIVE: char = 'q';
pub const SHORTHAND_QUADRATIC_BEZIER_ABSOLUTE: char = 'T';
pub const SHORTHAND_QUADRATIC_BEZIER_RELATIVE: char = 't';
pub const ELIPTICAL_ARC_ABSOLUTE: char = 'A';
pub const ELIPTICAL_ARC_RELATIVE: char = 'a';
pub const MOVE_ABSOLUTE: char = 'M';
pub const MOVE_RELATIVE: char = 'm';
pub const FINISH_PATH_LOWER: char = 'z';
pub const FINISH_PATH_UPPER: char = 'Z';


pub const QUADRATIC_POINTS_PER_GROUP: i32 = 2;
pub const CUBIC_POINTS_PER_GROUP: i32 = 3;
pub const SHORTHAND_CUBIC_POINTS_PER_GROUP: i32 = 2;
pub const ELIPTICAL_ARC_POINTS_PER_GROUP: i32 = 7; // Note: This one isnt counting x, y pairings - Austin Haskell