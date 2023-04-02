use ndarray::Array2;
pub enum CellState
{
    Dead,
    Alive
}

impl Default for CellState
{
    fn default() -> Self {
        CellState::Dead
    }
}

pub struct Field
{
    space: Array2::<CellState>,
}

impl Field {
    pub fn new(dimension: u32) -> Self {
        let dimension = dimension as usize;
        Self { 
            space: Array2::default((dimension, dimension))
        }
    }

    pub fn set(&mut self, state: CellState, row: u32, col: u32)
    {
        let row = row as usize;
        let col = col as usize;
        self.space[[row, col]] = state;
    }

}