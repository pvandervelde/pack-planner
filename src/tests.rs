use std::io::{Cursor, Write};

use crate::*;

// PackSortOrder

#[test]
fn when_parsing_sort_order_it_should_return_the_correct_type() {
    let natural_sort_order =
        PackSortOrder::from_str("NATURAL").expect("Failed to parse the natural sort order.");
    assert_eq!(PackSortOrder::Natural, natural_sort_order);

    let short_to_long_sort_order = PackSortOrder::from_str("SHORT_TO_LONG")
        .expect("Failed to parse the short-to-long sort order.");
    assert_eq!(PackSortOrder::ShortToLong, short_to_long_sort_order);

    let long_to_short_sort_order = PackSortOrder::from_str("LONG_TO_SHORT")
        .expect("Failed to parse the long-to-short sort order");
    assert_eq!(PackSortOrder::LongToShort, long_to_short_sort_order);
}

#[test]
fn when_creating_a_string_representation_of_the_sort_order_it_should_return_the_correct_value() {
    assert_eq!("NATURAL", PackSortOrder::Natural.to_string());
    assert_eq!("SHORT_TO_LONG", PackSortOrder::ShortToLong.to_string());
    assert_eq!("LONG_TO_SHORT", PackSortOrder::LongToShort.to_string());
}

// ItemTemplate

#[test]
fn when_parsing_a_valid_item_input_string_it_should_return_an_item_template() {
    let input = "item1,10.5,20,3.0";
    let result = ItemTemplate::from_str(input);
    assert!(result.is_ok());

    let item = result.unwrap();
    assert_eq!(item.id, "item1");
    assert_eq!(item.length, 10.5);
    assert_eq!(item.count, 20);
    assert_eq!(item.weight, 3.0);
}

#[test]
fn when_parsing_an_item_input_with_too_few_properties_it_should_return_an_error() {
    let input = "item1,10.5,20";
    let result = ItemTemplate::from_str(input);
    assert!(result.is_err());
    assert_eq!(
        result.err().unwrap(),
        Error::InvalidNumberOfPropertiesForItem {
            input: input.to_string(),
            property_count: 3
        }
    );
}

#[test]
fn when_parsing_an_item_input_with_too_many_properties_it_should_return_an_error() {
    let input = "item1,10.5,20,3.0,10.0";
    let result = ItemTemplate::from_str(input);
    assert!(result.is_err());
    assert_eq!(
        result.err().unwrap(),
        Error::InvalidNumberOfPropertiesForItem {
            input: input.to_string(),
            property_count: 5
        }
    );
}

#[test]
fn when_parsing_an_item_input_with_an_invalid_item_length_it_should_return_an_error() {
    let input = "item1,abc,20,3.0";
    let result = ItemTemplate::from_str(input);
    assert!(result.is_err());
}

#[test]
fn when_parsing_an_item_input_with_an_invalid_item_weight_it_should_return_an_error() {
    let input = "item1,10.5,20,xyz";
    let result = ItemTemplate::from_str(input);
    assert!(result.is_err());
}

#[test]
fn when_parsing_an_item_input_with_an_invalid_item_amount_it_should_return_an_error() {
    let input = "item1,10.5,abc,3.0";
    let result = ItemTemplate::from_str(input);
    assert!(result.is_err());
}

// PackTemplate

#[test]
fn when_creating_a_new_pack_template_it_should_initialize_properly() {
    let pack = PackTemplate::new();
    assert_eq!(pack.maximum_number_of_pieces, 0);
    assert_eq!(pack.maximum_weight, 0.0);
    assert_eq!(pack.sort_order, PackSortOrder::NotSet);
}

#[test]
fn when_parsing_a_valid_pack_input_string_it_should_return_a_pack_template() {
    let mut pack = PackTemplate::new();
    let input = "NATURAL,10,20.0";
    let result = pack.from_line(input);
    assert!(result.is_ok());
    assert_eq!(pack.maximum_number_of_pieces, 10);
    assert_eq!(pack.maximum_weight, 20.0);
    assert_eq!(pack.sort_order, PackSortOrder::Natural);
}

#[test]
fn when_parsing_a_pack_input_with_too_few_properties_it_should_return_an_error() {
    let mut pack = PackTemplate::new();
    let input = "NATURAL,10";
    let result = pack.from_line(input);
    assert!(result.is_err());
    assert_eq!(
        result.err().unwrap(),
        Error::InvalidNumberOfPropertiesForPacks {
            input: input.to_string(),
            property_count: 2
        }
    );
}

#[test]
fn when_parsing_a_pack_input_with_too_many_properties_it_should_return_an_error() {
    let mut pack = PackTemplate::new();
    let input = "NATURAL,10,20.0,Extra";
    let result = pack.from_line(input);
    assert!(result.is_err());
    assert_eq!(
        result.err().unwrap(),
        Error::InvalidNumberOfPropertiesForPacks {
            input: input.to_string(),
            property_count: 4
        }
    );
}

#[test]
fn when_parsing_a_pack_input_with_an_invalid_sort_order_it_should_return_an_error() {
    let mut pack = PackTemplate::new();
    let input = "InvalidSortOrder,10,20.0";
    let result = pack.from_line(input);
    assert!(result.is_err());
}

#[test]
fn when_parsing_a_pack_input_with_an_invalid_quantity_it_should_return_an_error() {
    let mut pack = PackTemplate::new();
    let input = "NATURAL,abc,20.0";
    let result = pack.from_line(input);
    assert!(result.is_err());
}

#[test]
fn when_parsing_a_pack_input_with_an_invalid_weight_it_should_return_an_error() {
    let mut pack = PackTemplate::new();
    let input = "NATURAL,10,abc";
    let result = pack.from_line(input);
    assert!(result.is_err());
}

// parse_input()

#[test]
fn when_parsing_a_valid_input_it_should_return_the_templates() {
    let input = "NATURAL,10,20.0\n100,10.5,20,3.0\n110,8.0,15,5.0";
    let mut cursor = Cursor::new(input);
    let result = parse_input(&mut cursor);
    assert!(result.is_ok());

    let (pack_template, item_templates) = result.unwrap();
    assert_eq!(pack_template.maximum_number_of_pieces, 10);
    assert_eq!(pack_template.maximum_weight, 20.0);
    assert_eq!(pack_template.sort_order, PackSortOrder::Natural);
    assert_eq!(item_templates.len(), 2);
    assert_eq!(item_templates[0].id, "100");
    assert_eq!(item_templates[1].id, "110");
}

#[test]
fn when_parsing_input_with_invalid_pack_information_it_should_return_an_error() {
    let input = "INVALID_KEYWORD,10,20.0\n100,10.5,20,3.0";
    let mut cursor = Cursor::new(input);
    let result = parse_input(&mut cursor);
    assert!(result.is_err());
}

#[test]
fn when_parsing_input_with_duplicate_pack_information_it_should_return_an_error() {
    let input = "NATURAL,10,20.0\nNATURAL,8,15.0\n100,10.5,20,3.0";
    let mut cursor = Cursor::new(input);
    let result = parse_input(&mut cursor);
    assert!(result.is_err());
}

#[test]
fn when_parsing_input_with_invalid_item_information_it_should_return_an_error() {
    let input = "NATURAL,10,20.0\ninvalid_item_format\n100,10.5,20,3.0";
    let mut cursor = Cursor::new(input);
    let result = parse_input(&mut cursor);
    assert!(result.is_err());
}

// maximum_number_of_items_to_add
#[test]
fn when_finding_the_maximum_items_with_a_weight_limit_it_should_return_the_correct_count() {
    let pack_template = PackTemplate {
        maximum_number_of_pieces: 10,
        maximum_weight: 50.0,
        sort_order: PackSortOrder::NotSet,
    };
    let current_pack_weight = 30.0;
    let current_pack_item_count = 5;
    let template = ItemTemplate {
        id: "item1".to_string(),
        length: 10.0,
        weight: 5.0,
        count: 1,
    };
    assert_eq!(
        maximum_number_of_items_to_add(
            &pack_template,
            current_pack_weight,
            current_pack_item_count,
            &template
        ),
        4
    );
}

#[test]
fn when_finding_the_maximum_items_with_an_item_limit_it_should_return_the_correct_count() {
    let pack_template = PackTemplate {
        maximum_number_of_pieces: 10,
        maximum_weight: 50.0,
        sort_order: PackSortOrder::NotSet,
    };
    let current_pack_weight = 20.0;
    let current_pack_item_count = 9;
    let template = ItemTemplate {
        id: "item1".to_string(),
        length: 10.0,
        weight: 5.0,
        count: 1,
    };
    assert_eq!(
        maximum_number_of_items_to_add(
            &pack_template,
            current_pack_weight,
            current_pack_item_count,
            &template
        ),
        1
    );
}

#[test]
fn when_finding_the_maximum_items_with_no_limit_it_should_return_the_correct_count() {
    let pack_template = PackTemplate {
        maximum_number_of_pieces: 10,
        maximum_weight: 50.0,
        sort_order: PackSortOrder::NotSet,
    };
    let current_pack_weight = 45.0;
    let current_pack_item_count = 9;
    let template = ItemTemplate {
        id: "item1".to_string(),
        length: 10.0,
        weight: 5.0,
        count: 1,
    };
    assert_eq!(
        maximum_number_of_items_to_add(
            &pack_template,
            current_pack_weight,
            current_pack_item_count,
            &template
        ),
        1
    );
}
