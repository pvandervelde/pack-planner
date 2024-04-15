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
#[derive(Clone)]
struct ItemTemplate {
    id: String,
    length: f64,
    weight: f64,
    count: i32,
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

        let length;
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

        let weight;
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

        let count;
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
    maximum_number_of_pieces: i32,
    maximum_weight: f64,
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

        let pack_sort_order;
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

        let maximum_number_of_items;
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

        let maximum_weight;
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

fn maximum_number_of_items_to_add(
    pack_template: &PackTemplate,
    current_pack_weight: f64,
    current_pack_item_count: i32,
    template: &ItemTemplate,
) -> i32 {
    let weight_space_in_pack = pack_template.maximum_weight - current_pack_weight;
    let item_space_in_pack = pack_template.maximum_number_of_pieces - current_pack_item_count;

    let max_items_by_weight = (weight_space_in_pack / template.weight).floor() as i32;
    let items_to_add = if max_items_by_weight < item_space_in_pack {
        max_items_by_weight
    } else {
        item_space_in_pack
    };
    items_to_add
}

fn print_item_line(item: &ItemTemplate, count: i32) {
    println!(
        "{},{:.1},{},{:.1}",
        item.id, item.length, count, item.weight
    );
}

fn print_footer(current_weight: f64, pack_length: f64) {
    println!("Pack Length: {pack_length:.1}, Pack Weight: {current_weight:.1}");
}

fn print_packs(items: Vec<ItemTemplate>, pack_template: PackTemplate) {
    let mut pack_index = 0;
    let mut current_pack_weight = pack_template.maximum_weight;
    let mut current_pack_item_count = pack_template.maximum_number_of_pieces;
    let mut longest_item_in_pack: f64 = 0.0;

    for (index, template) in items.iter().enumerate() {
        if template.weight > pack_template.maximum_weight {
            // Uh oh
            panic!("A single item weighs more than the maximum weight of the pack. We will never be able to add it.");
        }

        let mut items_left_from_current_batch = template.count;
        while items_left_from_current_batch > 0 {
            let mut items_to_add = maximum_number_of_items_to_add(
                &pack_template,
                current_pack_weight,
                current_pack_item_count,
                &template,
            );
            if items_to_add > 0 {
                let items_to_pack: i32;
                (items_to_pack, items_left_from_current_batch) =
                    if items_to_add < items_left_from_current_batch {
                        (items_to_add, items_left_from_current_batch - items_to_add)
                    } else {
                        (items_left_from_current_batch, 0)
                    };

                print_item_line(&template, items_to_pack);
                current_pack_weight += (items_to_pack as f64) * template.weight;
                current_pack_item_count += items_to_pack;

                longest_item_in_pack = if template.length > longest_item_in_pack {
                    template.length
                } else {
                    longest_item_in_pack
                };

                items_to_add -= items_to_pack;
            }

            if items_to_add <= 0 {
                // Print a summary if we have printed at least one pack
                if pack_index > 0 {
                    print_footer(current_pack_weight, longest_item_in_pack);
                    println!();
                }

                pack_index += 1;
                longest_item_in_pack = 0.0;
                current_pack_weight = 0.0;
                current_pack_item_count = 0;

                // print the header
                if index <= items.len() - 1 && items_left_from_current_batch > 0 {
                    println!("Pack Number: {pack_index}");
                }
            }
        }
    }

    //print_footer(current_pack_weight, longest_item_in_pack);
}

fn main() {
    let stdin = io::stdin();
    let (pack_template, item_templates) = parse_input(&mut stdin.lock()).expect("Parsing failure.");

    let items = match pack_template.sort_order {
        PackSortOrder::Natural => {
            // Do nothing. Just pass it through as it was
            item_templates
        }
        PackSortOrder::ShortToLong => {
            let mut sorted_order = item_templates.clone();
            sorted_order.sort_by(|a, b| {
                a.length
                    .partial_cmp(&b.length)
                    .expect("There shouldn't be any NaN's")
            });
            sorted_order
        }
        PackSortOrder::LongToShort => {
            let mut sorted_order = item_templates.clone();
            sorted_order.sort_by(|a, b| {
                b.length
                    .partial_cmp(&a.length)
                    .expect("There shouldn't be any NaN's")
            });
            sorted_order
        }
        _ => {
            // Error
            panic!("Undefined sort order detected.")
        }
    };

    print_packs(items, pack_template);
}
