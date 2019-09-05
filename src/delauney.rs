use core::intrinsics::sqrtf32;

use alloc::vec::Vec;

use crate::edge::Edge;
use crate::triangle::Triangle;
use crate::vertex::Vertex;

pub fn triangulation(vertices: &mut [Vertex]) -> Vec<Edge> {
    let n = vertices.len() + 3;

    let mut closed_triangles: Vec<Triangle> = Vec::with_capacity(2 * n - 5);

    let mut open_triangles: Vec<Triangle> = vec![];

    let enclosing_triangle = get_enclosing_triangle(vertices);

    open_triangles.push(enclosing_triangle.clone());

    vertices.sort_unstable();
    let (unique_vertices, _) = vertices.partition_dedup();

    let mut last_x = None;
    for vertex in unique_vertices.iter() {
        if let Some(x) = last_x {
            if x == vertex.x {
                continue;
            }
        }
        last_x = Some(vertex.x);
        // Create a list of edges to hold the edges of the triangles that will be modifed in this iteration.
        let mut edge_buffer: Vec<Edge> = Vec::new();

        // For each triangle in openTriangles, do two verification:
        // 1. If the triangle circumcircle is entirely to the left (along the x-axis) of current vertex, that triangle is done and add it to the triangulation.
        // 2. If currentVertex is its circumcircles, add its edges to edgeBuffer and remove it from openTriangles.
        open_triangles.retain(|triangle| {
            let circumcenter = triangle.get_circumcenter();
            if vertex.x > circumcenter.x
                && triangle.get_squared_circumradius()
                    < (vertex.x - circumcenter.x) * (vertex.x - circumcenter.x)
            {
                // To avoid having the enclosing triangle in final list.
                if !Triangle::has_shared_vertex(triangle, &enclosing_triangle) {
                    // It would be great to actually move the triangle instead of copying it.
                    closed_triangles.push(triangle.clone());
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
        // Find the edges in edgeBuffer that only appear once in the list.
        let unique_edges = get_unique_edges(&edge_buffer);
        // For each edge in uniqueEdgeBuffer, add a new triangle in openTriangles based on the edge and
        // currentVertex.
        open_triangles.extend(unique_edges.into_iter().map(|edge| Triangle {
            a: edge.a,
            b: edge.b,
            c: *vertex,
        }));
    }

    // Transfert the remaining triangles from openTriangles to the triangulation.
    // Because of the sweepline algorithm, the triangles whose circumcirle is not entirely left of the last
    // vertex were not transfered in closedTriangles.
    closed_triangles.extend(
        open_triangles
            .into_iter()
            .filter(|triangle| !Triangle::has_shared_vertex(&triangle, &enclosing_triangle)),
    );

    let all_edges: Vec<Edge> = closed_triangles
        .iter()
        .flat_map(|triangle| triangle.get_edges())
        .collect();
    get_dedup_edges(&all_edges)
}

struct Box {
    pub min: Vertex,
    pub max: Vertex,
}

impl Box {
    pub fn from_tuples(((min_x, min_y), (max_x, max_y)): ((f32, f32), (f32, f32))) -> Self {
        Box {
            min: Vertex { x: min_x, y: min_y },
            max: Vertex { x: max_x, y: max_y },
        }
    }

    pub fn get_extent(&self) -> Vertex {
        self.max - self.min
    }

    pub fn get_center(&self) -> Vertex {
        (self.max + self.min) / 2.0
    }
}

fn get_bounds(vertices: &[Vertex]) -> Box {
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
}

fn get_enclosing_triangle(vertices: &[Vertex]) -> Triangle {
    // This triangle could probably be tighter.
    let sqrt3 = unsafe { sqrtf32(3.0) };

    let bounds = get_bounds(vertices);

    let extent = bounds.get_extent();

    let center = bounds.get_center();

    // I could not use max, because it requires Ord...
    let triangle_radius = if extent.x >= extent.y {
        extent.x
    } else {
        extent.y
    };

    let triangle_size = triangle_radius * 2.0 * sqrt3;

    let triangle_height = triangle_size * sqrt3 / 2.0;

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

fn get_unique_edges(edges: &[Edge]) -> Vec<Edge> {
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
}

fn get_dedup_edges(edges: &[Edge]) -> Vec<Edge> {
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
}
