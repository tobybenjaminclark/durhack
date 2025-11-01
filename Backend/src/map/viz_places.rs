use plotters::prelude::*;
use std::error::Error;
use crate::types::Map;

pub fn viz_map(map: &Map) -> Result<(), Box<dyn Error>> {
    // Create drawing area
    let root = BitMapBackend::new("map.png", (800, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption("Map Visualization", ("sans-serif", 25))
        .build_cartesian_2d(-1.1f64..1.1f64, -1.1f64..1.1f64)?;

    chart.configure_mesh().disable_mesh().draw()?;

    // --- Draw routes as lines ---
    for (a, b) in &map.routes {
        chart.draw_series(LineSeries::new(vec![a.coords, b.coords], &BLUE))?;
    }

    // --- Plot locations ---
    for place in &map.locations {
        chart.draw_series(PointSeries::of_element(
            vec![place.coords],
            8,
            &GREEN,
            &|c, s, st| {
                EmptyElement::at(c)
                    + Circle::new((0, 0), s, st.filled())
                    + Text::new(place.name.clone(), (10, 0), ("sans-serif", 15).into_font())
            },
        ))?;
    }

    root.present()?;
    println!("✅ Map saved to {}", "map.png");
    Ok(())
}
