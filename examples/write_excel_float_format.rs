// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

//! An example of writing a Polar Rust dataframe to an Excel file. This
//! demonstrates setting an Excel number format for floats.

use polars::prelude::*;

fn main() {
    // Create a sample dataframe for the example.
    let df: DataFrame = df!(
        "Float" => &[1000.0, 2000.22, 3000.333, 4000.4444],
    )
    .unwrap();

    example(&df).unwrap();
}

use polars_excel_writer::PolarsXlsxWriter;

fn example(df: &DataFrame) -> PolarsResult<()> {
    let mut writer = PolarsXlsxWriter::new();

    writer.set_float_format("#,##0.00");

    writer.write_dataframe(df)?;
    writer.write_excel("dataframe.xlsx")?;

    Ok(())
}
