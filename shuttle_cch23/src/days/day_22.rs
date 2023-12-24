//!day_22.rs

use crate::app_error::{AppError, AppResult};
use axum::{body::Bytes, routing::post, Router};
use image::EncodableLayout;
use my_lib::my_binary_tree::BinaryTreeNode;
use my_lib::my_tree::TreeNode;

pub fn get_routes() -> Router {
    Router::new()
        .route("/22/integers", post(identify_single_integer))
        .route("/22/rocket", post(to_the_stars))
}

async fn identify_single_integer(data: Bytes) -> AppResult<String> {
    let input = String::from_utf8_lossy(data.as_bytes());
    let mut line_iter = input.lines();
    let tree_root = BinaryTreeNode::<usize>::new(
        line_iter
            .next()
            .unwrap()
            .parse::<usize>()
            .expect("bad input"),
    );
    for line in line_iter {
        tree_root.append_value(line.parse::<usize>().expect("bad input"));
    }
    match tree_root
        .iter_in_order_traversal()
        .filter(|n| n.get_count() == 1)
        .next()
    {
        Some(node) => {
            let presents = String::from_iter(vec!['🎁'; node.get_value()].iter());
            Ok(presents)
        }
        None => Err(AppError::bad_request("no single integer found")),
    }
}

#[derive(Default, Debug, PartialEq, Clone, Copy)]
struct Star {
    x: i32,
    y: i32,
    z: i32,
}

impl From<&str> for Star {
    fn from(value: &str) -> Self {
        let mut star = Star::default();
        let coordinates: Vec<i32> = value
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("bad line"))
            .collect();
        star.x = coordinates[0];
        star.y = coordinates[1];
        star.z = coordinates[2];
        star
    }
}

impl Star {
    fn distance(&self, other: Self) -> f32 {
        f32::sqrt(
            ((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2))
                as f32,
        )
    }
}

#[derive(Default, Debug)]
struct Portal {
    alpha: usize,
    omega: usize,
}

impl From<&str> for Portal {
    fn from(value: &str) -> Self {
        let mut portal = Portal::default();
        let indizes: Vec<usize> = value
            .split_whitespace()
            .map(|s| s.parse::<usize>().expect("bad line"))
            .collect();
        portal.alpha = indizes[0];
        portal.omega = indizes[1];
        portal
    }
}

async fn to_the_stars(data: Bytes) -> AppResult<String> {
    let input = String::from_utf8_lossy(data.as_bytes());
    let mut line_iter = input.lines();
    let num_stars = line_iter.next().unwrap().parse::<usize>()?;
    let mut stars: Vec<Star> = Vec::new();
    for _i in 0..num_stars {
        stars.push(Star::from(line_iter.next().unwrap()));
    }
    let num_portals = line_iter.next().unwrap().parse::<usize>()?;
    let portals: Vec<Portal> = line_iter.map(|line| Portal::from(line)).collect();
    // sanity check
    assert!(stars.len() == num_stars);
    assert!(portals.len() == num_portals);
    // fill tree from last star up through portals; each star is only allowed once
    let destination_star = *stars.last().unwrap();
    // expecting not more than 10 portals from each star
    let children_capacity = 10;
    let tree_root = TreeNode::seed_root(destination_star, children_capacity);
    let mut min_steps: usize = 0;
    for (node, level) in tree_root.iter_level_order_traversal() {
        let current_star = *node.get_value();
        if current_star == stars[0] {
            min_steps = level;
            break;
        }
        let index = stars.iter().position(|s| *s == current_star).unwrap();
        for portal in portals
            .iter()
            .filter(|p| p.alpha == index || p.omega == index)
        {
            let next_star_index = if portal.alpha == index {
                portal.omega
            } else {
                portal.alpha
            };
            node.add_unambiguous_child(stars[next_star_index], children_capacity);
        }
    }
    let start_star = tree_root.get_node(&stars[0]).unwrap();
    let mut distance: f32 = 0.0;
    let mut last_star = stars[0];
    for node in start_star.iter_back_track().skip(1) {
        let current_star = *node.get_value();
        distance += last_star.distance(current_star);
        last_star = current_star;
    }
    Ok(format!("{} {:.3}", min_steps, distance))
}
