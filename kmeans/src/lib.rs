use tera::{Context, Tera};
use std::str::FromStr;
use std::iter::Iterator;
use ndarray::{Array2};
use rand::Rng;

#[derive(Clone, Debug)]
pub struct Graph {
    pub name: String,
    pub size: usize,
    pub points: Vec<Point>,
    pub colour: String,
    pub x_range: f64,
    pub y_range: f64,
    pub x_min: f64,
    pub y_min: f64,
}

#[derive(Clone, Debug, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Graph {
    pub fn new(name: String, colour: String) -> Self {
        Graph {
            name,
            size: 0,
            points: Vec::new(),
            colour,
            x_range : 0.,
            y_range : 0.,
            x_min : 0.,
            y_min : 0.,
        }
    }

    pub fn add_point(&mut self, x: f64, y: f64) {
        self.points.push(Point { x, y });
    }

    pub fn draw_svg(
        &self, width: usize,
        height: usize,
        padding: usize,
        path: Vec<Point>,
        centers: Vec<(f64, f64)>) -> String {
        let mut context = Context::new();

        let mut p: Vec<(f64, f64)> = Vec::new();

        for point in path {
            p.push((point.x, point.y));
        }
        //let min_x = graph.points.get(0).map(|val| val.x).unwrap_or(0.0);
        /*let max_x = self
          .points
          .iter()
          .map(|point| point.x)
          .fold(0. / 0., f64::max);
        //let min_y = graph.points.iter().map(|val| val.y).fold(0. / 0., f64::min);
        let max_y = self
          .points
          .iter()
          .map(|point| point.y)
          .fold(0. / 0., f64::max);
          //hardset the padding around the graph
        */
        // let c_str = CString::new(file).unwrap();
        // let filename: *const c_char = c_str.as_ptr() as *const c_char;
        // const filename: &str = file.clone();

        //ensure the viewbox is as per input

        context.insert("name", &self.name);
        context.insert("width", &width);
        context.insert("height", &height);
        context.insert("padding", &padding);
        context.insert("path", &p);
        context.insert("centers", &centers);
        context.insert("x_range", &self.x_range);
        context.insert("y_range", &self.y_range);
        context.insert("x_min", &self.x_min);
        context.insert("y_min", &self.y_min);
        context.insert("colour", &self.colour);
        context.insert("lines", &10);

        Tera::one_off(include_str!("graph.svg"), &context, true).expect("Could not draw graph")
    }
}

pub fn generate_graph(xs: Vec<f64>, ys: Vec<f64>, title : &str) -> Graph {
    let mut graph = Graph::new(title.into(), "#8ff0a4".into());
    graph.size = xs.len();
    for i in 0..graph.size {
        graph.add_point(xs[i], ys[i]);
    }
    return graph;
}

pub fn fit_draw(
    num_points: usize,
    num_clusters: usize,
    pxmi: usize,
    pxma: usize,
    pymi: usize,
    pyma: usize,
    width: usize,
    height: usize,
    padding: usize,
    title: &str) -> String {
    let mut data: Vec<f64> = Vec::new();
    read_data(&mut data, num_points, pxmi, pxma, pymi, pyma);
    let mut xs: Vec<f64> = Vec::new();
    let mut ys: Vec<f64> = Vec::new();
    let mut tuples: Vec<(f64, f64)> = Vec::new();
    let mut centers: Vec<(f64, f64)> = Vec::new();

    let data2 = data.clone();
    let center_arr: Vec<f64> = fit(data2, num_clusters);
    println!("{:?}", center_arr);

    for i in 0..center_arr.len() {
        if (i % 2) == 1 {
            centers.push((center_arr[i-1], center_arr[i]));
        }
    }

    for i in 0..data.len() {
        if (i % 2) == 1 {
            tuples.push((data[i-1], data[i]));
        }
    }

    for i in 0..tuples.len() {
        xs.push(tuples[i].0);
        ys.push(tuples[i].1);
    }

    let mut graph = generate_graph(xs, ys, title);

    let width = width - padding * 2;
    let height = height - padding * 2;
    //let min_x = graph.points.get(0).map(|val| val.x).unwrap_or(0.0);
    let x_max = graph.points.iter().map(|point| point.x).fold(0. / 0., f64::max);
    let x_min = graph.points.iter().map(|point| point.x).fold(0. / 0., f64::min);
    let y_max = graph.points.iter().map(|point| point.y).fold(0. / 0., f64::max);
    let y_min = graph.points.iter().map(|point| point.y).fold(0. / 0., f64::min);

    graph.x_min = (x_min-1.0).round();
    graph.y_min = (y_min-1.0).round();

    graph.x_range = (x_max+1.0).round() - graph.x_min;
    graph.y_range = (y_max+1.0).round() - graph.y_min;

    //let min_y = graph.points.iter().map(|val| val.y).fold(0. / 0., f64::min);

    let centers = centers.iter().map(
        |val| ((val.0-graph.x_min) / graph.x_range * width as f64 + padding as f64,
        (val.1-graph.y_min) / graph.y_range * (height as f64 * -1.0) + (padding + height) as f64)
    ).collect();

    let path = graph.points.iter().map(|val| Point {
        //x: (val.x / graph.max_x * width as f64) + padding as f64,
        //y: (val.y / graph.max_y * (height as f64 * -1.0)) + (padding + height) as f64,
        x: ((val.x-graph.x_min) / graph.x_range * width as f64) + padding as f64,
        y: ((val.y-graph.y_min) / graph.y_range * (height as f64 * -1.0)) + (padding + height) as f64,
    }).collect();
    //  .enumerate()
    //  .map(|(i, point)| {
    //      if i == 0 {
    //          format!("M {} {}", point.x, point.y)
    //      } else {
    //          format!("L {} {}", point.x, point.y)
    //      }
    //  })
    //  .collect::<Vec<String>>().join(" ");

    let out = graph.draw_svg(width, height, padding, path, centers);
    return out;
}

fn read_data(
    data: &mut Vec<f64>,
    num_points: usize,
    pxmi: usize,
    pxma: usize,
    pymi: usize,
    pyma: usize,
    ) {
    // runtime generate num_points's number of points
    let mut rng = rand::thread_rng();

    let pxmi: f64 = pxmi as f64;
    let pxma: f64 = pxma as f64;
    let pymi: f64 = pymi as f64;
    let pyma: f64 = pyma as f64;

    for index in 0..num_points {
        data.push(rng.gen_range(pxmi, pxma));
        data.push(rng.gen_range(pymi, pyma));
    }
    println!("{:?}", data);
}

pub fn fit(data: Vec<f64>, num_clusters: usize) -> Vec<f64> {
    let arr = Array2::from_shape_vec((data.len() / 2, 2), data).unwrap();
    let (means, _clusters) = rkm::kmeans_lloyd(&arr.view(), num_clusters);

    let mut serialized_vec : Vec<f64> = Vec::new();
    for row in means.genrows() {
        serialized_vec.push(row[0]);
        serialized_vec.push(row[1]);
    }
    return serialized_vec;
}
