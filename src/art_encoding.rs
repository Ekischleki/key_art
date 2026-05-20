use std::array;

use fractional_bitstream::{
    frac::Frac, information_builder::InformationBuilder, information_stream::InformationStream,
};
use itertools::Itertools;

pub struct EncodingArt<'a> {
    data_bits: usize,
    stream: InformationStream<&'a [u8]>,
}

#[derive(Clone, Copy)]
pub enum GridPixel {
    Empty,
    Val(usize),
}
const PIXEL_VALUES: &[char] = &[' ', '.', '-', '=', '+', '*', '#', '%', '@'];

const BRIGHTNESS_LEVELS: usize = PIXEL_VALUES.len();
impl<'a> EncodingArt<'a> {
    pub fn new(data_bits: usize, stream: InformationStream<&'a [u8]>) -> Self {
        Self { data_bits, stream }
    }

    fn apply_kernel(grid: &Vec<Vec<GridPixel>>, pos: (isize, isize), radius: isize) -> Frac {
        let mut total_weight = Frac::zero();
        let mut total_val = Frac::zero();
        let (x, y) = pos;
        for (ox, oy) in ((-radius)..radius).cartesian_product(((-radius)..radius)) {
            let nx = x + ox;
            let ny = y + oy;
            let dist = (ox).pow(4) + (oy).pow(4);
            let v = grid.get(nx as usize).and_then(|r| r.get(ny as usize));
            if let Some(GridPixel::Val(v)) = v {
                let weight = Frac::from_usize(1, dist as usize);
                total_weight = &total_weight + &Frac::from_usize(1, dist as usize);
                total_val = &total_val + &(&weight * &Frac::from_usize(*v, 1))
            }
        }
        if &total_weight == &Frac::zero() {
            return Frac::zero();
        }
        &total_val / &total_weight
    }

    pub fn decode_img(grid: &str) -> Vec<u8> {
        let ref_grid_size = (grid.split('\n').next().unwrap().chars().count() - 2) / 2;
        let mut ref_grid = vec![vec![0usize; ref_grid_size]; ref_grid_size * 2];
        for (x, y, c) in grid
            .split('\n')
            .skip(1)
            .take(ref_grid_size)
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .skip(1)
                    .take(2 * ref_grid_size)
                    .enumerate()
                    .map(move |(x, c)| (x, y, c))
            })
            .flatten()
        {
            let brightness = PIXEL_VALUES.iter().position(|b| *b == c).unwrap_or(0);
            ref_grid[x][y] = brightness;
        }

        let mut encoder = fractional_bitstream::information_builder::InformationBuilder::new();

        let mut grid: Vec<Vec<GridPixel>> = vec![];
        let mut read_bits = Frac::zero();
        //let mut data_bits = Frac::from_usize(self.data_bits, 1);
        let mut grid_size = 0usize;
        let mut total_tiles = 0;
        let mut total_empty = 0;
        while grid_size < ref_grid_size {
            //Expand grid
            grid_size += 1;
            for col in &mut grid {
                col.push(GridPixel::Empty);
            }
            grid.push(vec![GridPixel::Empty; grid_size]);
            grid.push(vec![GridPixel::Empty; grid_size]);

            /*
            o o o 7 11
            o o o 6 10
            o o o 5 9
            1 2 3 4 5
             */

            for i in 0..(2 * grid_size) {
                let x = i;
                let y = grid_size - 1;
                let weighted_avg = Self::apply_kernel(
                    &grid,
                    (x as isize, y as isize),
                    (grid_size / 2).clamp(1, 5) as isize,
                );
                let distr = Self::gen_distr(weighted_avg, total_empty, total_tiles);
                let brightness = ref_grid[x][y];
                //println!("{:#?}\n{brightness}", distr);
                encoder.write_distr(&distr, brightness);
                grid[x][y] = GridPixel::Val(brightness);
                total_tiles += 1;
                if brightness == 0 {
                    total_empty += 1;
                }
            }
            for i in (0..grid_size - 1).rev() {
                let x = 2 * grid_size - 2;
                let y = i;
                let weighted_avg = Self::apply_kernel(
                    &grid,
                    (x as isize, y as isize),
                    (grid_size / 2).clamp(1, 3) as isize,
                );
                let distr = Self::gen_distr(weighted_avg, total_empty, total_tiles);
                let brightness = ref_grid[x][y];
                encoder.write_distr(&distr, brightness);
                grid[x][y] = GridPixel::Val(brightness);
                total_tiles += 1;
                if brightness == 0 {
                    total_empty += 1;
                }
            }
            for i in (0..grid_size - 1).rev() {
                let x = 2 * grid_size - 1;
                let y = i;
                let weighted_avg = Self::apply_kernel(
                    &grid,
                    (x as isize, y as isize),
                    (grid_size / 2).clamp(1, 3) as isize,
                );
                let distr = Self::gen_distr(weighted_avg, total_empty, total_tiles);
                let brightness = ref_grid[x][y];
                encoder.write_distr(&distr, brightness);
                grid[x][y] = GridPixel::Val(brightness);
                total_tiles += 1;
                if brightness == 0 {
                    total_empty += 1;
                }
            }
        }

        let mut res = vec![];

        encoder.write_to_stream(&mut res).unwrap();
        res
    }

    pub fn encode_image(mut self, size: usize) -> String {
        let mut grid: Vec<Vec<GridPixel>> = vec![];
        let mut read_bits = Frac::zero();
        let mut data_bits = Frac::from_usize(self.data_bits, 1);
        let mut grid_size = 0usize;
        let mut total_tiles = 0;
        let mut total_empty = 0;
        while grid_size < size {
            //Expand grid
            grid_size += 1;
            for col in &mut grid {
                col.push(GridPixel::Empty);
            }
            grid.push(vec![GridPixel::Empty; grid_size]);
            grid.push(vec![GridPixel::Empty; grid_size]);

            /*
            o o 5 6
            1 2 3 4
             */

            for i in 0..(2 * grid_size) {
                let x = i;
                let y = grid_size - 1;
                let weighted_avg = Self::apply_kernel(
                    &grid,
                    (x as isize, y as isize),
                    (grid_size / 2).clamp(1, 5) as isize,
                );
                let distr = Self::gen_distr(weighted_avg, total_empty, total_tiles);
                let brightness = self.stream.get_from_distr(&distr);
                //println!("{:#?}\n{brightness}", distr);
                grid[x][y] = GridPixel::Val(brightness);
                total_tiles += 1;
                if brightness == 0 {
                    total_empty += 1;
                }
            }
            for i in (0..grid_size - 1).rev() {
                let x = 2 * grid_size - 2;
                let y = i;
                let weighted_avg = Self::apply_kernel(
                    &grid,
                    (x as isize, y as isize),
                    (grid_size / 2).clamp(1, 3) as isize,
                );
                let distr = Self::gen_distr(weighted_avg, total_empty, total_tiles);
                let brightness = self.stream.get_from_distr(&distr);
                grid[x][y] = GridPixel::Val(brightness);
                total_tiles += 1;
                if brightness == 0 {
                    total_empty += 1;
                }
            }
            for i in (0..grid_size - 1).rev() {
                let x = 2 * grid_size - 1;
                let y = i;
                let weighted_avg = Self::apply_kernel(
                    &grid,
                    (x as isize, y as isize),
                    (grid_size / 2).clamp(1, 3) as isize,
                );
                let distr = Self::gen_distr(weighted_avg, total_empty, total_tiles);
                let brightness = self.stream.get_from_distr(&distr);
                grid[x][y] = GridPixel::Val(brightness);
                total_tiles += 1;
                if brightness == 0 {
                    total_empty += 1;
                }
            }
        }

        let mut buf = String::new();
        buf.push('╭');
        buf.push_str(&"─".repeat(grid_size * 2));
        buf.push_str("╮\n");

        for y in 0..grid_size {
            buf.push('│');
            for x in 0..(grid_size * 2) {
                let brightness = match grid[x][y] {
                    GridPixel::Val(v) => v,
                    _ => panic!(),
                };
                buf.push(PIXEL_VALUES[brightness]);
            }
            buf.push_str("│\n");
        }
        buf.push('╰');
        buf.push_str(&"─".repeat(grid_size * 2));
        buf.push_str("╯");

        //println!("{buf}");
        buf
    }

    fn gen_distr(f: Frac, total_empty: usize, total_tiles: usize) -> [Frac; BRIGHTNESS_LEVELS] {
        let mut res = array::from_fn(|_| Frac::zero());
        let mut total_weight = Frac::zero();
        for i in 0..BRIGHTNESS_LEVELS {
            let pos = Frac::from_usize(i, 1);
            let dist = if pos > f { &pos - &f } else { &f - &pos };
            let weight = &(&Frac::one() / &(&dist + &Frac::from_usize(1, 16)))
                + &Frac::from_usize(4, i * 10 + 1);
            let weight = weight.round_nearest(&Frac::from_usize(1, 0xFF));
            total_weight = &total_weight + &weight;
            res[i] = weight;
        }
        if f == Frac::zero() && Frac::from_usize(total_empty, total_tiles) > Frac::from_usize(4, 5)
        {
            res[1] = &res[1] + &Frac::from_usize(1000, 1);
            total_weight = &total_weight + &Frac::from_usize(1000, 1);
        }
        for f in &mut res {
            *f = &*f / &total_weight;
            //println!("{:#?}", f);
        }
        let sum = res.iter().fold(Frac::zero(), |a, b| &a + b);
        assert_eq!(sum, Frac::one());
        res
    }
}

pub fn gen_encoding(bytes: &[u8]) -> Result<String, &'static str> {
    if bytes.len() > 128 {
        return Err("Byte length is too big");
    }
    let bytes = [[bytes.len() as u8].as_slice(), bytes].concat();
    for size in 1..32 {
        let encoder = EncodingArt::new(0, InformationStream::new(&bytes));
        let img = encoder.encode_image(size);
        let mut r = EncodingArt::decode_img(&img);
        if r.len() == 0 && bytes.len() == 1 {
            return Ok(img);
        }
        if r.len() == 0 {
            continue;
        }
        if r[0] as usize != bytes.len() - 1 {
            continue;
        }
        r.resize_with(r[0] as usize + 1, || 0);
        if r == bytes {
            return Ok(img);
        }
    }

    return Err("Couldn't encode the bytes");
}

pub fn decode_image(image: &str) -> Vec<u8> {
    let mut r = EncodingArt::decode_img(&image);
    r.resize_with(r[0] as usize + 1, || 0);
    r
}
