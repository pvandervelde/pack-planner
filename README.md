# Pack Planner

A small application for sorting items into different packs based on their weight and quantity.

## Requirements

An item is represented by an ID, a length, a weight and the quantity. A pack is represented by a
pack ID, a weight limit and a maximum number of items it can contain.

Packs are created by stacking items in one of three orders:

* In the order they were given, aka `NATURAL`
* From shortest to longest, aka `SHORT_TO_LONG`
* From longest to shortest, aka `LONG_TO_SHORT`

## Create the environment

This application is build using Rust 1.76.0. To install Rust, follow the instructions at
[https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install). Once Rust is installed
you should be able to call

    cargo build

from the workspace directory to build the application. By default it will end up in `target/debug/pack-planner`, on a
Unix system. Or in `target/debug/pack-planner.exe` on a Windows system.

## Usage

Once you have build the application you can run it using

    target/debug/pack-planner

When the application starts it will wait for user input. You can add as much input as you like. The
program will process the input once you enter an empty line. The program will then output the
packing list.

The input is expected to look like

    [Sort order],[max pieces per pack],[max weight per pack]
    [item id],[item length],[item quantity],[piece weight]
    [item id],[item length],[item quantity],[piece weight]
    [item id],[item length],[item quantity],[piece weight]

for instance

    NATURAL,40,500.0
    1001,6200,30,9.653
    2001,7200,50,11.21

The output will look like

    Pack number: [pack number]
    [item id],[item length],[item quantity],[piece weight]
    [item id],[item length],[item quantity],[piece weight]

Given the above input this will look like

    Pack Number: 1
    1001,6200,30,9.653
    2001,7200,10,11.21
    Pack Length: 7200, Pack Weight: 401.69

    Pack Number: 2
    2001,7200,40,11.21
    Pack Length: 7200, Pack Weight: 448.4

## Testing

There are a number of unit tests in the `test.rs` file. You can run these using:

    cargo test
