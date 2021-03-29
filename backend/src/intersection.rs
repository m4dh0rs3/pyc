use crate::graph::Graph;
use crate::Board;
use crate::Curve;
use math::Vec2D;

impl Board {
    pub(crate) fn split_curves(&mut self) {
        let mut i = 0;

        'curves: while i < self.graph.edges.len() {
            let curve_len = self.graph.edges[i].1.path.len();
            let mut j = 0;

            while j < self.graph.edges.len() {
                if i.max(j) - i.min(j) > 1 {
                    if &self.graph.edges[i].1 != &self.graph.edges[j].1 {
                        let mut k = 0;
                        let other_len = self.graph.edges[j].1.path.len();

                        while k + 1 < curve_len {
                            let mut l = 0;

                            while l + 1 < other_len {
                                if let Some(intersection) = segment_intersection(
                                    self.graph.edges[i].1.path[k],
                                    self.graph.edges[i].1.path[k + 1],
                                    self.graph.edges[j].1.path[l],
                                    self.graph.edges[j].1.path[l + 1],
                                ) {
                                    let j = if j > i { j - 1 } else { j };

                                    let ((curve_start, curve_end), curve) =
                                        self.graph.edges.remove(i);
                                    let ((other_start, other_end), other) =
                                        self.graph.edges.remove(j);

                                    let (curve_one, curve_two) = curve.path.split_at(k);
                                    let (curve_one, curve_two) = (
                                        Curve {
                                            start: curve.start,
                                            mid: curve.mid,
                                            end: curve.end,
                                            path: {
                                                let mut curve_one = curve_one.to_vec();
                                                /* if k == 0 {
                                                    curve_one.insert(0, curve.start.into());
                                                } */
                                                curve_one.push(intersection);
                                                curve_one
                                            },
                                        },
                                        Curve {
                                            start: curve.start,
                                            mid: curve.mid,
                                            end: curve.end,
                                            path: {
                                                let mut curve_tow = curve_two.to_vec();
                                                curve_tow[0] = intersection;
                                                curve_tow
                                            },
                                        },
                                    );

                                    let (other_one, other_two) = other.path.split_at(l);
                                    let (other_one, other_two) = (
                                        Curve {
                                            start: other.start,
                                            mid: other.mid,
                                            end: other.end,
                                            path: {
                                                let mut other_one = other_one.to_vec();
                                                /* if l == 0 {
                                                    other_one.insert(0, other.start.into());
                                                } */
                                                other_one.push(intersection);
                                                other_one
                                            },
                                        },
                                        Curve {
                                            start: other.start,
                                            mid: other.mid,
                                            end: other.end,
                                            path: {
                                                let mut other_two = other_two.to_vec();
                                                other_two[0] = intersection;
                                                other_two
                                            },
                                        },
                                    );

                                    let node = self.graph.push_node(intersection);
                                    self.graph.insert_edge_at(i, node, curve_end, curve_two);
                                    self.graph.insert_edge_at(i, curve_start, node, curve_one);
                                    self.graph.insert_edge_at(j, node, other_end, other_two);
                                    self.graph.insert_edge_at(j, other_start, node, other_one);

                                    i = 0;
                                    //j -= 2;

                                    continue 'curves;
                                }

                                l += 1;
                            }

                            k += 1;
                        }
                    }
                }

                j += 1;
            }

            i += 1;
        }
    }
}

fn segment_intersection(
    p1: Vec2D<f64>,
    p2: Vec2D<f64>,
    r1: Vec2D<f64>,
    r2: Vec2D<f64>,
) -> Option<Vec2D<f64>> {
    let s1 = p2 - p1;
    let s2 = r2 - r1;

    let k = s1.cross_zero(s2);

    if k == 0.into() {
        return None;
    }

    let d = p1 - r1;

    let s = s1.cross_zero(d) / k;
    let t = s2.cross_zero(d) / k;

    /* let s = (-s1.y * (p1.x - r1.x) + s1.x * (p1.y - r1.y)) / k;
    let t = (s2.x * (p1.y - r1.y) - s2.y * (p1.x - r1.x)) / k; */

    if s > 0.into() && s < 1.into() && t > 0.into() && t < 1.into() {
        Some(Vec2D::new(p1.x + t * s1.x, p1.y + t * s1.y))
    } else {
        None
    }
}

fn aabb_intersection(p1: Vec2D<f64>, p2: Vec2D<f64>, r1: Vec2D<f64>, r2: Vec2D<f64>) -> bool {
    p1.x <= r2.x && p2.x >= r1.x && p1.y <= r2.y && p2.y >= r1.y
}
