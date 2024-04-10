use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

use plotters::prelude::*;
const OUT_FILE_NAME: &str = "histogram.png";

#[derive(Debug, Clone)]
struct Data {
    size: f64,
    count: f64,
}

fn size_frequency(file: File) -> Box<[Data]> {
    let mut sizes = Vec::new();

    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let mut str = line.split_whitespace();
        let count = str.next().unwrap().parse().unwrap();
        let size = str.next().unwrap().parse().unwrap();
        sizes.push(Data { size, count });
    }
    sizes.into()
}
fn max_count(sizes: &Box<[Data]>) -> f64 {
    let mut max = 0.;
    for i in sizes.iter() {
        if max < i.count {
            max = i.count;
        }
    }
    max
}
fn percent_in_range(data: &Box<[Data]>, min: f64, max: f64) -> f64 {
    let mut total_amount = 0.;
    let mut in_range_amount = 0.;
    for i in data.iter() {
        total_amount += i.count;
        if min <= i.size && i.size <= max {
            in_range_amount += i.count;
        }
    }
    println!("Total amount of files: {}",total_amount);
    println!("Files in range: {}",in_range_amount);
    let percent = in_range_amount * 100. / total_amount;
    println!(
        "{}% of files are in range between {} to {} bytes",
        percent, min, max
    );
    percent
}

fn draw_histogram(data: &Box<[Data]>) -> Result<(), Box<dyn std::error::Error>> {
    let backend = BitMapBackend::new(OUT_FILE_NAME, (1920, 1080)).into_drawing_area();
    backend.fill(&WHITE)?;

    let y_axis_max = max_count(data);
    let x_spec = 0.1f64..1_000_000 as f64;
    let y_spec = 0.1f64..(y_axis_max + 1000.) as f64;
    let mut ctx = ChartBuilder::on(&backend)
        .caption(
            "Гістограма кількості файлів залежно від їх розміру",
            ("sans-serif", 40).into_font(),
        )
        .margin(15)
        .set_all_label_area_size(30)
        .build_cartesian_2d(
            x_spec.log_scale().zero_point(0.),
            y_spec.log_scale().zero_point(0.),
        )?;

    ctx.configure_mesh()
        .y_desc("Number of files")
        .x_desc("File size")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    ctx.draw_series(data.iter().map(|e| {
        let points = [
            (e.size as f64 - 0.45, 0.0001),
            (e.size as f64 + 0.45, e.count as f64),
        ];
        let bar = Rectangle::new(points, RED.filled());
        bar
    }))?;

    percent_in_range(&data, 0., 20_000.);

    //let l = [(20_000., 0.0001), (20_000., (y_axis_max + 10_000.) as f64)];
    //ctx.draw_series(LineSeries::new(l.iter().map(|e| *e), &BLUE))?;
    ctx.draw_series((0..1).into_iter().map(|_| {
        Rectangle::new(
            [
                (20_000. - 200., 0.0001),
                (20_000. + 200., y_axis_max + 10_000.),
            ],
            BLUE.filled(),
        )
    }))?;

    backend.present()?;
    Ok(())
}

fn main() -> Result<(), io::Error> {
    let mut args = env::args();
    let _program_name = args.next();

    let file = args.next().unwrap_or("result.txt".to_string());
    let file = Path::new(&file);
    let file = File::open(file)?;

    let data = size_frequency(file);
    let _ = draw_histogram(&data);
    //match size_frequency(file) {
    //    Ok(sizes) => {
    //        //dbg!(&sizes);
    //        dbg!(&sizes.len());
    //        let _ = draw_histogram(sizes);
    //    }
    //    Err(error) => {
    //        println!("{}", error);
    //    }
    //}
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_result() {
        let file = "result.txt".to_string();
        let file = Path::new(&file);
        let file = File::open(file).unwrap();

        let default = size_frequency(file);
        let mut sorted = default.clone();
        sorted.sort_by_key(|e| e.size);

        for (l, r) in default.iter().zip(sorted.iter()) {
            assert_eq!(l.size, r.size);
            assert_eq!(l.count, r.count);
        }
    }
}
