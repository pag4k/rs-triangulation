use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

use std::fmt::Debug;

use crate::edge::Edge;
use crate::triangle::Triangle;
use crate::vertex::Vertex;

pub fn triangulation<T>(vertices: &[Vertex<T>]) -> Vec<Edge<T>>
where
    T: Copy
        + PartialOrd
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Mul<f64, Output = T>
        + Div<f64, Output = T>
        + Debug,
    f64: From<T>,
{
    let mut closed_triangles: Vec<Triangle<T>> = vec![];

    let mut open_triangles: Vec<Triangle<T>> = vec![];

    // Assume we have at least 3 triangles.

    //let mut n = vertices.len();

    let enclosing_triangle = get_enclosing_triangle(&vertices);

    //dbg!(&enclosing_triangle);

    //n += 3;

    open_triangles.push(enclosing_triangle);

    // Check if there are not duplicates?

    let mut sorted_vertices = vertices.to_vec();
    sorted_vertices.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

    //dbg!(&sorted_vertices);

    for vertex in sorted_vertices.iter() {
        // Create a list of edges to hold the edges of the triangles that will be modifed in this iteration.
        let mut edge_buffer: Vec<Edge<T>> = Vec::new();

        //dbg!(&vertex);

        // For each triangle in openTriangles, do two verification:
        // 1. If the triangle circumcircle is entirely to the left (along the x-axis) of current vertex, that triangle is done and add it to the triangulation.
        // 2. If currentVertex is its circumcircles, add its edges to edgeBuffer and remove it from openTriangles.
        open_triangles.retain(|triangle| {
            if triangle.get_squared_circumradius()
                - f64::from(
                    (vertex.x - triangle.get_circumcenter().x)
                        * (vertex.x - triangle.get_circumcenter().x),
                )
                < 0.0
            {
                // To avoid having the enclosing triangle in final list.
                if !Triangle::has_shared_vertex(triangle, &enclosing_triangle) {
                    // It would be great to actually move the triangle instead of copying it.
                    closed_triangles.push(*triangle);
                }
                false
            } else if triangle.is_in_circumcircle(vertex) {
                edge_buffer.push(Edge::new(triangle.a, triangle.b));
                edge_buffer.push(Edge::new(triangle.b, triangle.c));
                edge_buffer.push(Edge::new(triangle.c, triangle.a));
                false
            } else {
                true
            }
        });
        //dbg!(&edge_buffer);
        // Find the edges in edgeBuffer that only appear once in the list.
        let unique_edges = get_unique_edges(&edge_buffer);
        //dbg!(&unique_edges);
        // For each edge in uniqueEdgeBuffer, add a new triangle in openTriangles based on the edge and
        // currentVertex.
        for edge in unique_edges.into_iter() {
            open_triangles.push(Triangle {
                a: edge.a,
                b: edge.b,
                c: *vertex,
            });
        }
    }
    //dbg!(&closed_triangles);
    //dbg!(&open_triangles);
    // Transfert the remaining triangles from openTriangles to the triangulation.
    // Because of the sweepline algorithm, the triangles whose circumcirle is not entirely left of the last
    // vertex were not transfered in closedTriangles.

    for triangle in open_triangles.iter() {
        if !Triangle::has_shared_vertex(triangle, &enclosing_triangle) {
            // It would be great to actually move the triangle instead of copying it.
            closed_triangles.push(*triangle);
        }
    }
    //    closed_triangles.append(&mut open_triangles);

    //closed_triangles

    let all_edges: Vec<Edge<T>> = closed_triangles
        .iter()
        .flat_map(|triangle| triangle.get_edges())
        .collect();
    get_dedup_edges(&all_edges)
}

struct Box<T>
where
    T: Copy,
{
    pub min: Vertex<T>,
    pub max: Vertex<T>,
}

impl<T> Box<T>
where
    T: Copy,
{
    pub fn from_tuples(((min_x, min_y), (max_x, max_y)): ((T, T), (T, T))) -> Self {
        Box {
            min: Vertex { x: min_x, y: min_y },
            max: Vertex { x: max_x, y: max_y },
        }
    }

    pub fn get_extent(&self) -> Vertex<T>
    where
        T: Sub<Output = T>,
    {
        self.max - self.min
    }

    pub fn get_center(&self) -> Vertex<T>
    where
        T: Add<Output = T> + Div<f64, Output = T>,
    {
        (self.max + self.min) / 2.0
    }
}

fn get_bounds<T: PartialOrd>(vertices: &[Vertex<T>]) -> Box<T>
where
    T: Copy,
{
    // We assume there is at least one element.
    Box::from_tuples(vertices.iter().fold(
        (
            (vertices[0].x, vertices[0].x),
            (vertices[0].x, vertices[0].y),
        ),
        |((min_x, min_y), (max_x, max_y)), vertex| {
            (
                (
                    if vertex.x < min_x { vertex.x } else { min_x },
                    if vertex.y < min_y { vertex.y } else { min_y },
                ),
                (
                    if vertex.x > max_x { vertex.x } else { max_x },
                    if vertex.y > max_y { vertex.y } else { max_y },
                ),
            )
        },
    ))

    // let min_x = vertices.iter().fold(vertices[0].x, |min_x, vertex| if vertex.x < min_x {vertex.x} else {min_x} );
    // let max_x = vertices.iter().fold(vertices[0].x, |max_x, vertex| if vertex.x > max_x {vertex.x} else {max_x} );
    // let min_y = vertices.iter().fold(vertices[0].y, |min_y, vertex| if vertex.y < min_y {vertex.y} else {min_y} );
    // let max_y = vertices.iter().fold(vertices[0].y, |max_y, vertex| if vertex.y > max_y {vertex.y} else {max_y} );
}

fn get_enclosing_triangle<T>(vertices: &[Vertex<T>]) -> Triangle<T>
where
    T: Copy
        + PartialOrd
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<f64, Output = T>
        + Div<f64, Output = T>,
{
    // This triangle could probably be tighter.
    let bounds = get_bounds(vertices);

    let extent = bounds.get_extent();

    let center = bounds.get_center();

    // I could not use max, because it requires Ord...
    let triangle_radius = if extent.x >= extent.y {
        extent.x
    } else {
        extent.y
    };

    let triangle_size = triangle_radius * 2.0 * f64::sqrt(3.0);

    let triangle_height = triangle_size * f64::sqrt(3.0) / 2.0;

    Triangle {
        a: Vertex {
            x: center.x,
            y: center.y + triangle_height - triangle_radius,
        },
        b: Vertex {
            x: center.x - triangle_size / 2.0,
            y: center.y - triangle_radius,
        },
        c: Vertex {
            x: center.x + triangle_size / 2.0,
            y: center.y - triangle_radius,
        },
    }
}

fn get_unique_edges<T>(edges: &[Edge<T>]) -> Vec<Edge<T>>
where
    T: PartialEq + Clone,
{
    let mut unique_indices = Vec::new();

    for (i, edge1) in edges.iter().enumerate() {
        if edges.iter().filter(|&edge2| edge1 == edge2).count() == 1 {
            unique_indices.push(i);
        }
    }

    unique_indices
        .iter()
        .map(|&index| edges[index].clone())
        .collect()
    // let mut map: HashMap<Edge<T>, usize> = HashMap::new();
    // // It would be nice to avoid the clone.
    // for edge in edges.to_vec() {
    //     let counter = map.entry(edge).or_insert(0);
    //     *counter += 1;
    // }
    // map.into_iter()
    //     .filter_map(|(key, value)| if value == 1 as usize { Some(key) } else { None })
    //     .collect()
}

fn get_dedup_edges<T>(edges: &[Edge<T>]) -> Vec<Edge<T>>
where
    T: PartialEq + Clone,
{
    let mut unique_indices = Vec::new();

    for (i, edge1) in edges.iter().enumerate() {
        if unique_indices
            .iter()
            .map(|&index| &edges[index])
            .filter(|edge2| edge1 == *edge2)
            .count()
            == 0
        {
            unique_indices.push(i);
        }
    }

    unique_indices
        .iter()
        .map(|&index| edges[index].clone())
        .collect()
    // let mut map: HashMap<Edge<T>, usize> = HashMap::new();
    // // It would be nice to avoid the clone.
    // for edge in edges.to_vec() {
    //     let counter = map.entry(edge).or_insert(0);
    //     *counter += 1;
    // }
    // map.into_iter()
    //     .filter_map(|(key, value)| if value == 1 as usize { Some(key) } else { None })
    //     .collect()
}
