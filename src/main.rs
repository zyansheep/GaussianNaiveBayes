use std::error::Error;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashMap;

use statrs::distribution::{Normal, Univariate};
use statrs::statistics::Statistics;

#[derive(Debug, Clone)]
pub struct Point {
	x: f64,
	y: f64,
}

#[derive(Debug, Clone)]
pub struct Class {
	id: i32,
	points: Vec<Point>,
	normal_distribution: Option<(Normal, Normal)>,
}
impl Class {
	fn new(id: i32, points: Vec<Point>) -> Class {
		Class {
			id, points,
			normal_distribution: None,
		}
	}
	fn generate_distribution(&mut self) -> Result<(), statrs::StatsError> {
		self.normal_distribution = Some({
			println!("points: {:?}", self.points);
			let x_data = self.points.iter().map(|point| point.x).collect::<Vec<f64>>();
			let y_data = self.points.iter().map(|point| point.y).collect::<Vec<f64>>();
			println!("x_data, y_data: {:?}, {:?}", x_data, y_data);
			(
				Normal::new(x_data.clone().mean(), x_data.std_dev())?,
				Normal::new(y_data.clone().mean(), y_data.std_dev())?,
			)
		});
		println!("Distribution for class {}: {:?}", self.id, self.normal_distribution);
		Ok(())
	}
	fn get_likelyhood(&self, initial: f64, point: &Point) -> Option<f64> {
		use std::f64; 
		if let Some(distribution) = self.normal_distribution {
			return Some(
				initial.ln()
				+ distribution.0.cdf(point.x).ln()
				+ distribution.1.cdf(point.y).ln()
			)
		} else { return None }
	}
}
#[derive(Debug, Clone)]
pub struct ClassData {
	classes: HashMap<i32, Class>
}
impl ClassData {
	fn parse_from_reader(data: impl BufRead) -> Result<ClassData, Box<dyn Error>> {
		let mut classes: HashMap<i32, Class> = HashMap::new();
		for line in data.lines() {
			let line = line?;
			let mut line_iter = line.split_whitespace();
			// Parse id and point data
			let id = 
				if let Some(id) = line_iter.next() {
					id.parse::<i32>()?
				} else { break; };
			
			let point = 
				if let (Some(x_str), Some(y_str)) = (line_iter.next(), line_iter.next()) {
					Point { x: x_str.parse::<f64>()?, y: y_str.parse::<f64>()? }
				} else { break; };
			
			// Insert class / class data
			if let Some(class) = classes.get_mut(&id) {
				class.points.push(point);
			} else {
				classes.insert( id, Class::new(id, vec![point]) );
			}
			
		}
		// generate distributions
		for (_,class) in &mut classes { class.generate_distribution()?; }
		Ok(ClassData {
			classes
		})
	}
}

fn main() {
	let input_path = ::std::env::args().nth(1).expect("Please pass first argument");
	let file = BufReader::new(File::open(&input_path).expect("Pleaes pass valid path as first argument"));
	let data = ClassData::parse_from_reader(file).expect("Failed to parse file format, make sure it is a space-separated list");
	
	let test = Point{x: 6.98645, y: -2.936};
	
	println!("Testing data: {:?}", test);
	println!("Class 0 log Probability: e^{:?}", data.classes[&0].get_likelyhood(0.5, &test).unwrap());
	println!("Class 1 log Probability: e^{:?}", data.classes[&1].get_likelyhood(0.5, &test).unwrap());
	
	// Computer distributions for each classes 2 data points: x and y
	
}
