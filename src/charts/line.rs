struct DataPoint {
    value: f64,
}

pub fn line_chart() -> String {
    let data = [
        DataPoint { value: 90.0 },
        DataPoint { value: 92.0 },
        DataPoint { value: 91.0 },
        DataPoint { value: 93.0 },
        DataPoint { value: 92.5 },
        DataPoint { value: 94.0 },
        DataPoint { value: 92.0 },
    ];

    let width = 120.0;
    let height = 50.0;
    let padding = 5.0;

    let values = data.iter().map(|d| d.value).collect::<Vec<f64>>();
    let min_value = values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_value = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    let points = data
        .iter()
        .enumerate()
        .map(|(i, d)| {
            let x = (i as f64 / (data.len() - 1) as f64) * (width - padding * 2.0) + padding;
            let y = height
                - padding
                - ((d.value - min_value) / (max_value - min_value)) * (height - padding * 2.0);
            (x, y)
        })
        .collect::<Vec<(f64, f64)>>();

    let mut path = format!("M {} {}", points[0].0, points[0].1);

    for i in 0..points.len() - 1 {
        let current = points[i];
        let next = points[i + 1];
        let mid_x = (current.0 + next.0) / 2.0;
        let mid_y = (current.1 + next.0) / 2.0;

        path.push_str(&format!(
            " Q {} {}, {} {}",
            current.0, current.1, mid_x, mid_y
        ));
    }

    let last_point = points[points.len() - 1];
    path.push_str(&format!(" L {} {}", last_point.0, last_point.1));

    // Note: CSS Animations or Web-sys is used for the "pathLength" effect
    // as there is no 1:1 "motion" library equivalent in Rust yet.
    format!(
        "<svg
            width=\"{0}\"
            height=\"{1}\"
            viewBox=\"0 0 {0} {1}\"
            xmlns=\"http://www.w3.org/2000/svg\"
        >
            <path
                d=\"{2}\"
                fill=\"none\"
                stroke=\"#000\"
                stroke-width=\"2.5\"
                stroke-linecap=\"round\"
                stroke-linejoin=\"round\"
                class=\"animate-path\"
            />
        </svg>
        ",
        width, height, path
    )
}
