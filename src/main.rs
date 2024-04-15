use std::io::{self, BufRead};
use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;
use std::string::ToString;

use strum::{Display, EnumString, ParseError};
use thiserror::Error;

#[cfg(test)]
#[path = "tests.rs"]
mod tests;

/// Defines the different errors for the swerve model crate.
#[derive(Debug, Error, PartialEq)]
pub enum Error {
    /// Indicates that one of the input strings didn't start in a valid way.
    ///
    /// * 'input' - The input string that was invalid.
    #[error("The provided input string {input:?} was not valid. Expected the string to start with a number or one of [NATURAL, SHORT_TO_LONG, LONG_TO_SHORT].")]
    InputStringShouldStartWithNumberOrKeyWord {
        /// The input string that is incorrect
        input: String,
    },

    /// Indicates that the current input string contains a pack information header, but the current line is not the first line. So duplicate information is
    /// provided.
    ///
    /// * 'current_line' - The contents of the current line, which contains the duplicate header
    /// * 'current_line_index' - The index of the current line.
    #[error("The provided input string {current_line:?} contains pack information, but this line is not the first line of the input stream. It is the {current_line_index:?} line. This means there is duplicate header information.")]
    InputContainsDuplicatePackInformation {
        current_line: String,
        current_line_index: usize,
    },

    /// Indicates that a string containing item information has too few or too many property values.
    ///
    /// * 'input' - The input string
    /// * 'property_count' - The number of property values that were found in the input string
    #[error("The provided input string {input:?} contains too few or too many property values. Expecting 3 values, but got {property_count:?}")]
    InvalidNumberOfPropertiesForPacks {
        input: String,
        property_count: usize,
    },

    // Indicates that a string containing pack information has an invalid value for the sort order.
    ///
    /// * 'input' - The input string
    /// * 'property_value' - The string containing the 'value' for the sort order
    /// * 'source' - The source error
    #[error("The provided input string {input:?} contains an invalid value for the sort order of a pack: {property_value:?}. Expected one of [NATURAL, SHORT_TO_LONG, LONG_TO_SHORT].")]
    InvalidPackSortOrder {
        input: String,
        property_value: String,
        #[source]
        source: ParseError,
    },

    /// Indicates that a string containing pack information has an invalid value for the number of items in a pack.
    ///
    /// * 'input' - The input string
    /// * 'property_value' - The string containing the 'value' for the number of items
    /// * 'source' - The source error
    #[error("The provided input string {input:?} contains an invalid value for the number of the items in a pack: {property_value:?}. Expected a positive integer number.")]
    InvalidPackItemCount {
        input: String,
        property_value: String,
        #[source]
        source: ParseIntError,
    },

    // Indicates that a string containing pack information has an invalid value for the weight of the pack.
    ///
    /// * 'input' - The input string
    /// * 'property_value' - The string containing the 'value' for the weight property
    /// * 'source' - The source error
    #[error("The provided input string {input:?} contains an invalid value for the weight of a pack: {property_value:?}. Expected a positive floating point number.")]
    InvalidPackWeight {
        input: String,
        property_value: String,
        #[source]
        source: ParseFloatError,
    },

    /// Indicates that a string containing item information has too few or too many property values.
    ///
    /// * 'input' - The input string
    /// * 'property_count' - The number of property values that were found in the input string
    #[error("The provided input string {input:?} contains too few or too many property values. Expecting 4 values, but got {property_count:?}")]
    InvalidNumberOfPropertiesForItem {
        input: String,
        property_count: usize,
    },

    /// Indicates that a string containing item information has an invalid value for the length of the item.
    ///
    /// * 'input' - The input string
    /// * 'property_value' - The string containing the 'value' for the length property
    /// * 'source' - The source error
    #[error("The provided input string {input:?} contains an invalid value for the length of the item: {property_value:?}. Expected a positive floating point number.")]
    InvalidItemLength {
        input: String,
        property_value: String,
        #[source]
        source: ParseFloatError,
    },

    /// Indicates that a string containing item information has an invalid value for the weight of the item.
    ///
    /// * 'input' - The input string
    /// * 'property_value' - The string containing the 'value' for the weight property
    /// * 'source' - The source error
    #[error("The provided input string {input:?} contains an invalid value for the weight of the item: {property_value:?}. Expected a positive floating point number.")]
    InvalidItemWeight {
        input: String,
        property_value: String,
        #[source]
        source: ParseFloatError,
    },

    /// Indicates that a string containing item information has an invalid value for the number of items.
    ///
    /// * 'input' - The input string
    /// * 'property_value' - The string containing the 'value' for the number of items
    /// * 'source' - The source error
    #[error("The provided input string {input:?} contains an invalid value for the number of the items: {property_value:?}. Expected a positive integer number.")]
    InvalidItemCount {
        input: String,
        property_value: String,
        #[source]
        source: ParseIntError,
    },
}

// Indices used when parsing the pack information from the input
const PACK_SORT_ORDER_INDEX: usize = 0;
const PACK_MAXIMUM_ITEM_COUNT_INDEX: usize = 1;
const PACK_MAXIMUM_WEIGHT_INDEX: usize = 2;

// Indices used when parsing the items from the input
const ITEM_ID_INDEX: usize = 0;
const ITEM_LENGTH_INDEX: usize = 1;
const ITEM_QUANTITY_INDEX: usize = 2;
const ITEM_WEIGHT_INDEX: usize = 3;

/// Contains properties for an item and the number of items with these properties as provided in the input.
struct ItemTemplate {
    id: String,
    length: f32,
    weight: f32,
    count: usize,
}

impl FromStr for ItemTemplate {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 4 {
            return Err(Error::InvalidNumberOfPropertiesForItem {
                input: line.to_string(),
                property_count: parts.len(),
            });
        }

        let id = parts[ITEM_ID_INDEX].to_string();

        let mut length: f32 = 0.0;
        match parts[ITEM_LENGTH_INDEX].parse() {
            Ok(v) => length = v,
            Err(e) => {
                return Err(Error::InvalidItemLength {
                    input: line.to_string(),
                    property_value: parts[ITEM_LENGTH_INDEX].to_string(),
                    source: e,
                })
            }
        };

        let mut weight = 0.0;
        match parts[ITEM_WEIGHT_INDEX].parse() {
            Ok(v) => weight = v,
            Err(e) => {
                return Err(Error::InvalidItemWeight {
                    input: line.to_string(),
                    property_value: parts[ITEM_WEIGHT_INDEX].to_string(),
                    source: e,
                })
            }
        };

        let mut count = 0;
        match parts[ITEM_QUANTITY_INDEX].parse() {
            Ok(v) => count = v,
            Err(e) => {
                return Err(Error::InvalidItemCount {
                    input: line.to_string(),
                    property_value: parts[ITEM_QUANTITY_INDEX].to_string(),
                    source: e,
                })
            }
        };

        Ok(Self {
            id,
            length,
            weight,
            count,
        })
    }
}

/// Defines the different ways in which packs can be ordered.
#[derive(Clone, Copy, Debug, Display, EnumString, PartialEq)]
enum PackSortOrder {
    NotSet,
    #[strum(to_string = "NATURAL")]
    Natural,
    #[strum(to_string = "SHORT_TO_LONG")]
    ShortToLong,
    #[strum(to_string = "LONG_TO_SHORT")]
    LongToShort,
}

struct PackTemplate {
    maximum_number_of_pieces: usize,
    maximum_weight: f32,
    sort_order: PackSortOrder,
}

impl PackTemplate {
    fn new() -> PackTemplate {
        PackTemplate {
            maximum_number_of_pieces: 0,
            maximum_weight: 0.0,
            sort_order: PackSortOrder::NotSet,
        }
    }

    fn from_line(&mut self, s: &str) -> Result<(), Error> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 3 {
            return Err(Error::InvalidNumberOfPropertiesForPacks {
                input: s.to_string(),
                property_count: parts.len(),
            });
        }

        let mut pack_sort_order = PackSortOrder::NotSet;
        match PackSortOrder::from_str(parts[PACK_SORT_ORDER_INDEX]) {
            Ok(s) => pack_sort_order = s,
            Err(e) => {
                return Err(Error::InvalidPackSortOrder {
                    input: s.to_string(),
                    property_value: parts[PACK_SORT_ORDER_INDEX].to_string(),
                    source: e,
                })
            }
        };

        let mut maximum_number_of_items = 0;
        match parts[PACK_MAXIMUM_ITEM_COUNT_INDEX].parse() {
            Ok(v) => maximum_number_of_items = v,
            Err(e) => {
                return Err(Error::InvalidPackItemCount {
                    input: s.to_string(),
                    property_value: parts[PACK_MAXIMUM_ITEM_COUNT_INDEX].to_string(),
                    source: e,
                })
            }
        };

        let mut maximum_weight = 0.0;
        match parts[PACK_MAXIMUM_WEIGHT_INDEX].parse() {
            Ok(v) => maximum_weight = v,
            Err(e) => {
                return Err(Error::InvalidPackWeight {
                    input: s.to_string(),
                    property_value: parts[PACK_MAXIMUM_WEIGHT_INDEX].to_string(),
                    source: e,
                })
            }
        };

        self.maximum_number_of_pieces = maximum_number_of_items;
        self.maximum_weight = maximum_weight;
        self.sort_order = pack_sort_order;

        Ok(())
    }
}

struct Pack {
    maximum_number_of_pieces: usize,
    maximum_weight: f32,
    //items: Vec<Item>,
}

impl Pack {
    // Number of pieces
    // Space left
    // Weight left
}

fn parse_input<R: BufRead>(reader: &mut R) -> Result<(PackTemplate, Vec<ItemTemplate>), Error> {
    let mut pack_template = PackTemplate::new();
    let mut item_templates: Vec<ItemTemplate> = Vec::new();

    let mut is_first_line = true;
    let mut line_index: usize = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }

        let trimmed_line = line.trim();

        // The line should start either with an integer number or one of [NATURAL, SHORT_TO_LONG, LONG_TO_SHORT]
        let is_number = if let Some(c) = trimmed_line.chars().next() {
            c.is_digit(10)
        } else {
            false // Empty string
        };

        let is_keyword = trimmed_line.starts_with("NATURAL")
            || trimmed_line.starts_with("SHORT_TO_LONG")
            || trimmed_line.starts_with("LONG_TO_SHORT");

        if !is_number && !is_keyword {
            return Err(Error::InputStringShouldStartWithNumberOrKeyWord {
                input: line.clone(),
            });
        }

        // The first line contains information about the packs that we're allowed to create
        if is_keyword {
            if !is_first_line {
                return Err(Error::InputContainsDuplicatePackInformation {
                    current_line: line,
                    current_line_index: line_index,
                });
            }

            pack_template.from_line(trimmed_line)?;

            // Any line after this cannot be the first line anymore.
            is_first_line = false;
        } else {
            let item = ItemTemplate::from_str(trimmed_line)?;
            item_templates.push(item);
        }

        line_index += 1;
    }

    Ok((pack_template, item_templates))
}

fn main() {
    let stdin = io::stdin();
    let (pack_template, item_templates) = parse_input(&mut stdin.lock()).expect("Parsing failure.");

    // Now do something with the templates.
}
