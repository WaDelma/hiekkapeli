
use rayon::prelude::*;
use std::fmt;
use std::mem;
#[derive(Clone, Copy, PartialEq, Hash, Debug)]
enum Tile {
    Air { pressure: i8 },
    Sand { humidity: u8 },
    Water { pressure: i8 },
}

impl fmt::Display for Tile {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::Tile::*;
        match self {
            Air { .. } => fmt.write_str(" "),
            Sand { .. } => fmt.write_str("#"),
            Water { .. } => fmt.write_str("~"),
        }
    }
}

mod render;

fn main() {
    render::start();
    let width = 211; //640;
    let height = 57; //480;

    let index = |x: usize, y: usize| x * height + y;

    let mut prev_buffer = vec![Tile::Air { pressure: 0 }; width * height];
    let mut cur_buffer = vec![Tile::Air { pressure: 0 }; width * height];

    for x in 0..width {
        prev_buffer[index(x, 0)] = Tile::Sand { humidity: 0 };
        prev_buffer[index(x, height - 1)] = Tile::Water { pressure: 0 };
    }
    for y in 0..height {
        prev_buffer[index(0, y)] = Tile::Sand { humidity: 0 };
        prev_buffer[index(width - 1, y)] = Tile::Water { pressure: 0 };
    }
    loop {
        // TODO: This allocation should be able to be reused
        let mut bufs = Vec::with_capacity(height);
        let mut buffer_slice = &mut cur_buffer[..];
        for _ in 0..width {
            let (cur, next) = buffer_slice.split_at_mut(height);
            buffer_slice = next;
            bufs.push(cur);
        }
        bufs.par_iter_mut().enumerate().for_each(|(x, b)| {
            for (y, t) in b.iter_mut().enumerate() {
                use self::Tile::*;
                match prev_buffer[index(x, y)] {
                    Air { pressure } => {
                        *t = Air { pressure };
                    }
                    Sand { humidity } => {
                        *t = Sand { humidity };
                    }
                    Water { pressure } => {
                        *t = Water { pressure };
                    }
                }
            }
        });
        mem::swap(&mut cur_buffer, &mut prev_buffer);

        for y in 0..height {
            for x in 0..width {
                print!("{}", prev_buffer[index(x, y)]);
            }
            println!();
        }
    }
}