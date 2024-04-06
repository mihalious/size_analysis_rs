use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::num::ParseFloatError;
use std::path::Path;

use plotters::prelude::*;
const OUT_FILE_NAME: &str = "plotters-doc-data/histogram.png";

fn parse_size_to_bytes(mut size: String) -> Result<f64, ParseFloatError> {
    match size.pop() {
        Some('K') => {
            let kb = size.parse::<f64>()?;
            Ok(kb * 1024.)
        }
        Some('M') => {
            let mb = size.parse::<f64>()?;
            Ok(mb * 1024. * 1024.)
        }
        Some('G') => {
            let gb = size.parse::<f64>()?;
            Ok(gb * 1024. * 1024. * 1024.)
        }
        Some(n) if n.is_ascii_digit() => {
            size.push(n);
            size.parse::<f64>()
        }
        _ => unreachable!("impossible filesize"),
    }
}

fn size_frequency(file: File) -> Result<BTreeMap<u64, u64>, io::Error> {
    let mut sizes = BTreeMap::new();

    let reader = BufReader::new(file);
    for line in reader.lines() {
        //match parse_size_to_bytes(line?) {
        //    Ok(size) => {
        //        sizes
        //            .entry(size)
        //            .and_modify(|counter| *counter += 1)
        //            .or_insert(1);
        //    }
        //    Err(e) => println!("{:?}", e),
        //}
        if let Ok(size) = parse_size_to_bytes(line?) {
            sizes
                .entry(size as u64)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
    }
    return Ok(sizes);
}

fn draw_histogram(sizes: BTreeMap<u64, u64>) -> Result<(), Box<dyn std::error::Error>> {
    let root_drawing_area = BitMapBackend::new(OUT_FILE_NAME, (640, 480)).into_drawing_area();
    root_drawing_area.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root_drawing_area)
        // Set the caption of the chart
        .caption("This is our first plot", ("sans-serif", 40).into_font())
        // Set the size of the label region
        .x_label_area_size(20)
        .y_label_area_size(40)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_cartesian_2d(0f32..10f32, 0f32..10f32)?;

    //for (size, number) in sizes {
    //    let size_f64: f64 = size.into();
    //    let size_log = size_f64.log2();
    //    println!("{}: {}", size_log, number);
    //}
    Ok(())
}

fn main() -> Result<(), io::Error> {
    let mut args = env::args();
    let _program_name = args.next();

    let file = args.next().unwrap_or("result.txt".to_string());
    let file = Path::new(&file);
    let file = File::open(file)?;

    match size_frequency(file) {
        Ok(sizes) => {
            dbg!(&sizes);
            dbg!(&sizes.len());
            dbg!(&sizes.into_values().sum::<u64>());

            //let _ = draw_histogram(sizes);
        }
        Err(error) => {
            println!("{}", error);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_size() {
        let kb = String::from("1K");
        assert_eq!(parse_size_to_bytes(kb), Ok(1024.));

        let kb_f = String::from("4.3K");
        assert_eq!(parse_size_to_bytes(kb_f), Ok(4.3 * 1024.));

        let mb = String::from("1M");
        assert_eq!(parse_size_to_bytes(mb), Ok(1_048_576.));

        let gb = String::from("1G");
        assert_eq!(parse_size_to_bytes(gb), Ok(1_073_741_824.));

        let gb_f = String::from("1.9G");
        assert_eq!(parse_size_to_bytes(gb_f), Ok(1.9 * 1_073_741_824.));

        let bt = String::from("123");
        assert_eq!(parse_size_to_bytes(bt), Ok(123.));
    }
}
