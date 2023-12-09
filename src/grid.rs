#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }
}

impl From<(i64, i64)> for Point {
    fn from(value: (i64, i64)) -> Self {
        Point::new(value.0, value.1)
    }
}

impl From<(&i64, &i64)> for Point {
    fn from(value: (&i64, &i64)) -> Self {
        Point::new(*value.0, *value.1)
    }
}
impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Point::new(value.0 as i64, value.1 as i64)
    }
}

impl From<(&usize, &usize)> for Point {
    fn from(value: (&usize, &usize)) -> Self {
        Point::new(*value.0 as i64, *value.1 as i64)
    }
}

#[derive(Debug)]
pub struct Grid2D<T> {
    pub grid: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid2D<T>
where
    T: PartialEq + Eq,
{
    pub fn in_bounds(&self, pt: Point) -> bool {
        let (w, h) = (self.width as i64, self.height as i64);
        pt.x >= 0 && pt.y >= 0 && pt.x < w && pt.y < h
    }

    fn idx(&self, pt: Point) -> usize {
        let (x, y) = (pt.x as usize, pt.y as usize);
        y * self.width + x
    }

    pub fn get(&self, pt: Point) -> Option<&T> {
        if !self.in_bounds(pt) {
            return None;
        }
        Some(&self.grid[self.idx(pt)])
    }

    pub fn get_mut(&mut self, pt: Point) -> Option<&mut T> {
        if !self.in_bounds(pt) {
            return None;
        }
        let idx = self.idx(pt);
        Some(&mut self.grid[idx])
    }

    pub fn contains(&self, item: T) -> bool {
        self.grid.contains(&item)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.grid.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.grid.iter_mut()
    }
}

impl From<&str> for Grid2D<char> {
    fn from(value: &str) -> Self {
        let width = value.lines().next().unwrap().len();
        let iter = value.lines().collect::<Vec<&str>>();
        let height = iter.len();
        let grid: Vec<char> = iter.into_iter().flat_map(|c| c.chars()).collect();
        Grid2D {
            grid,
            width,
            height,
        }
    }
}
