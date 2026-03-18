use mini_charts::charts::radar::RadarChart;
use mini_charts::{Mocha, Series, charts};
use mini_svg_macro::svg;

fn main() {
    let portfolio_a = [0.8, 0.6, 0.9, 0.5, 0.7, 0.4];
    let portfolio_b = [0.6, 0.7, 0.5, 0.8, 0.6, 0.5];
    let portfolio_c = [0.1, 0.3, 0.7, 0.2, 0.9, 0.8];

    let mut radar = RadarChart::new(1000);
    radar.add_series(Series {
        name: "Alpha".into(),
        values: portfolio_a.to_vec(),
        color: Mocha::RED.to_owned(),
    });

    radar.add_series(Series {
        name: "Beta".into(),
        values: portfolio_b.to_vec(),
        color: Mocha::BLUE.to_owned(),
    });
    radar.add_series(Series {
        name: "Theta".into(),
        values: portfolio_c.to_vec(),
        color: Mocha::GREEN.to_owned(),
    });

    let labels = vec![
        "Speed".into(),
        "Power".into(),
        "Agility".into(),
        "Stamina".into(),
        "Range".into(),
        "Will".into(),
    ];
    radar.set_axes_labels(labels);

    let svg_output = radar.render();
    std::fs::write("radar.svg", svg_output).expect("Unable to write file");
    println!("wrote SVG to radar.svg.");

    let s = "that".to_string();

    let svg = svg! {
        width: 100,
        hight: 100,

        comment!("this {} {}", s, "and this too!")

            rect {
                x:"10",
                y:"20",
            }
        polygon {
            fill: "red",
        }
    };

    println!("{svg}");
    let line_graph = charts::line::line_chart();
    std::fs::write("line.svg", line_graph).expect("Unable to write file");
    
}
