use crate::solver::{solve_step, step_config};
use crate::Visibility::{Any, BadFace, BadPiece};
use crate::{
    Algorithm, Solvable, CORNER_OPPOSITE_E_SLICE, CORNER_OPPOSITE_M_SLICE,
    CORNER_OPPOSITE_S_SLICE, EDGE_OPPOSITE_E_SLICE, EDGE_OPPOSITE_M_SLICE,
    EDGE_OPPOSITE_S_SLICE,
};
use cubelib::cube::Cube333;
use cubelib::defs::{NissSwitchType, StepKind};
use cubelib::steps::coord::Coord;
use cubelib::steps::finish::coords::HTRFinishCoord;
use pyo3::PyResult;

pub struct FinishLeaveSlice {
    pub axis: String,
    edge_opposite: &'static [u8; 12],
    corner_opposite: &'static [u8; 8],
}

impl FinishLeaveSlice {
    pub fn new(axis: &str) -> Self {
        let axis = if axis == "rl" { "lr" } else { axis };
        let (edge_opposite, corner_opposite) = match axis {
            "ud" => (&EDGE_OPPOSITE_E_SLICE, &CORNER_OPPOSITE_E_SLICE),
            "fb" => (&EDGE_OPPOSITE_S_SLICE, &CORNER_OPPOSITE_S_SLICE),
            "lr" => (&EDGE_OPPOSITE_M_SLICE, &CORNER_OPPOSITE_M_SLICE),
            _ => (&EDGE_OPPOSITE_E_SLICE, &CORNER_OPPOSITE_E_SLICE),
        };
        FinishLeaveSlice {
            axis: axis.to_string(),
            edge_opposite,
            corner_opposite,
        }
    }
}

impl Solvable for FinishLeaveSlice {
    fn is_solved(&self, cube: &Cube333) -> bool {
        let edges = cube.edges.get_edges();
        let corners = cube.corners.get_corners();
        let edges_ok = edges
            .iter()
            .enumerate()
            .all(|(i, e)| e.id as usize == i || e.id == self.edge_opposite[i]);
        let corners_ok = corners
            .iter()
            .enumerate()
            .all(|(i, c)| c.id as usize == i || c.id == self.corner_opposite[i]);
        edges_ok && corners_ok
    }

    fn is_eligible(&self, _cube: &Cube333) -> bool {
        true
    }

    fn case_name(&self, cube: &Cube333) -> String {
        let edges = cube.edges.get_edges();
        let corners = cube.corners.get_corners();
        let bad_edge_count = edges
            .iter()
            .enumerate()
            .filter(|(i, e)| e.id as usize != *i && e.id != self.edge_opposite[*i])
            .count();
        let bad_corner_count = corners
            .iter()
            .enumerate()
            .filter(|(i, c)| c.id as usize != *i && c.id != self.corner_opposite[*i])
            .count();
        let c_string = if bad_corner_count > 0 {
            format!("{}c", bad_corner_count)
        } else {
            "".to_string()
        };
        let e_string = if bad_edge_count > 0 {
            format!("{}e", bad_edge_count)
        } else {
            "".to_string()
        };
        format!("{}{}", c_string, e_string)
    }

    fn edge_visibility(&self, cube: &Cube333, pos: usize, _facelet: u8) -> u8 {
        let mut v = Any as u8;
        let e = cube.edges.get_edges()[pos];
        if e.id as usize != pos && e.id != self.edge_opposite[pos] {
            v |= BadPiece as u8 | BadFace as u8;
        }
        v
    }

    fn corner_visibility(&self, cube: &Cube333, pos: usize, _facelet: u8) -> u8 {
        let mut v = Any as u8;
        let c = cube.corners.get_corners()[pos];
        if c.id as usize != pos && c.id != self.corner_opposite[pos] {
            v |= BadPiece as u8 | BadFace as u8;
        }
        v
    }
    fn solve(&self, cube: &Cube333, count: usize) -> PyResult<Vec<Algorithm>> {
        let mut cfg = step_config(StepKind::FINLS, &self.axis, NissSwitchType::Never);
        cfg.max = Some(20);
        solve_step(cube, cfg, count, false)
    }
}

pub struct Finish;
impl Solvable for Finish {
    fn is_solved(&self, cube: &Cube333) -> bool {
        HTRFinishCoord::from(cube).val() == 0
    }

    fn is_eligible(&self, _cube: &Cube333) -> bool {
        true
    }

    fn case_name(&self, cube: &Cube333) -> String {
        let edges = cube.edges.get_edges();
        let corners = cube.corners.get_corners();
        let bad_edge_count = edges
            .iter()
            .enumerate()
            .filter(|(i, e)| (**e).id as usize != *i)
            .count();
        let bad_corner_count = corners
            .iter()
            .enumerate()
            .filter(|(i, c)| (**c).id as usize != *i)
            .count();
        let c_string = if bad_corner_count > 0 {
            format!("{}c", bad_corner_count)
        } else {
            "".to_string()
        };
        let e_string = if bad_edge_count > 0 {
            format!("{}e", bad_edge_count)
        } else {
            "".to_string()
        };
        format!("{}{}", c_string, e_string)
    }

    fn edge_visibility(&self, cube: &Cube333, pos: usize, _facelet: u8) -> u8 {
        let mut v = Any as u8;
        if cube.edges.get_edges()[pos].id as usize != pos {
            v |= BadPiece as u8 | BadFace as u8;
        }
        v
    }

    fn corner_visibility(&self, cube: &Cube333, pos: usize, _facelet: u8) -> u8 {
        let mut v = Any as u8;
        if cube.corners.get_corners()[pos].id as usize != pos {
            v |= BadPiece as u8 | BadFace as u8;
        }
        v
    }
    fn solve(&self, cube: &Cube333, count: usize) -> PyResult<Vec<Algorithm>> {
        let mut cfg = step_config(StepKind::FIN, "", NissSwitchType::Never);
        cfg.max = Some(20);
        solve_step(cube, cfg, count, false)
    }
}

#[cfg(test)]
mod tests {
    use crate::finish::Finish;
    use crate::{Cube, Solvable};

    #[test]
    fn htr_to_finish() {
        let mut cube = Cube::new("U' F2 U2 L2 U' R2 U F2 L2 R' U' F B' R D2 U' F R2 F U R2 B2 U2 R2 L2 F2 R2 U2 R2 B R2 F' L' F' R' U' F B D' R' F L' U L B2 U R2 F2 L".to_string()).unwrap().0;
        let finish = Finish;
        let solutions = finish.solve(&cube, 2).unwrap();
        assert!(solutions.len() > 0);
    }
}
