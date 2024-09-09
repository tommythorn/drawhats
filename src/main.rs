/*
 * First, map hexagons to a grid
 *
 *     >---<         >---<         >
 *    /     \       /     \       /
 *   /  0 0  \     /  2 0  \     /
 *  <    +    >---<    +    >---<
 *   \       /     \       /     \
 *    \     /  1 0  \     /  3 0  \
 *     >---<    +    >---<    +    >
 *    /     \       /     \       /
 *   /  0 1  \     /  2 1  \     /
 *  <    +    >---<    +    >---<
 *   \       /     \       /     \
 *    \     /  1 1  \     /  3 1  \
 *     >---<    +    >---<    +    >
 *    /     \       /     \       /
 *   /  0 2  \     /  2 2  \     /
 *  <    +    >---<    +    >---<
 *   \       /     \       /     \
 *    \     /  1 2  \     /  3 2  \
 *     >---<    +    >---<    +    >
 *
 * which gives rise to a funny neigbor offsets:
 * even columns: (0,-1), (1, -1), (1, 0), (0,1), (-1, 0), (-1, -1)
 * odd columns:  (0,-1), (1,  0), (1, 1), (0,1), (-1, 1), (-1,  0)
 */

use iterm2canvas::Pict;

// Grid width and height

const WHITE: u32 = 0xFFFFFF;
const YELLOW: u32 = 0xFFFF00;

const N: isize = 10;

const RADIUS: isize = 32;

#[derive(Copy, Clone, PartialEq, Debug)]
struct Hex(isize, isize);

impl Hex {
    fn screen(self) -> (isize, isize) {
        let y = self.1 * RADIUS * 2 + (if self.0 % 2 == 1 { RADIUS } else { 0 });
        let x = self.0 * RADIUS * 15 / 10;

        (x, y)
    }

    fn neighbor(self, direction: usize) -> Hex {
        let Hex(x, y) = self;
        let odd = if x % 2 == 1 { 1 } else { 0 };
        let (xd, yd) = match direction {
            0 => (0, -1),
            1 => (1, odd - 1),
            2 => (1, odd),
            3 => (0, 1),
            4 => (-1, odd),
            5 => (-1, odd - 1),
            _ => panic!("Illegal direction {direction}, must be [0..5]"),
        };

        Hex(x + xd, y + yd)
    }
}

mod test {
    use super::*;
    #[allow(dead_code)]
    fn all_neighbors(h: (isize, isize)) -> Vec<(isize, isize)> {
        (0..6)
            .map(|d| {
                let n = Hex(h.0, h.1).neighbor(d);
                (n.0, n.1)
            })
            .collect()
    }

    #[test]
    fn test_neighbor() {
        assert_eq!(
            all_neighbors((2, 1)),
            [(2, 0), (3, 0), (3, 1), (2, 2), (1, 1), (1, 0)]
        );
        assert_eq!(
            all_neighbors((1, 1)),
            [(1, 0), (2, 1), (2, 2), (1, 2), (0, 2), (0, 1)]
        );
    }
}

fn avg(p1: (isize, isize), p2: (isize, isize)) -> (isize, isize) {
    ((p1.0 + p2.0) / 2, (p1.1 + p2.1) / 2)
}

fn _avg3(p1: (isize, isize), p2: (isize, isize), p3: (isize, isize)) -> (isize, isize) {
    ((p1.0 + p2.0 + p3.0) / 3, (p1.1 + p2.1 + p3.1) / 3)
}

fn main() {
    let mut hex = [[[' '; 6]; N as usize]; N as usize];

    // Highlight a hat
    hex[2][2][1] = '*';
    hex[2][2][2] = '*';
    hex[3][2][4] = '*';
    hex[3][2][3] = '*';
    hex[2][3][0] = '*';
    hex[2][3][1] = '*';
    hex[2][3][4] = '*';
    hex[2][3][5] = '*';

    const WIDTH: isize = 640;
    let mut pict = Pict::new(WIDTH, WIDTH);

    // I just need the center point of hexagon h
    // The midpoint between the neighbor h' and h AVG(h',h) is the middle of the line between them.
    // Call the left neighbor of h and h' L(h,h') and R(h,h') then the avg of h, h', and L(

    for y in 0..N {
        for x in 0..N {
            let h = Hex(x, y);

            pict.plot(h.screen(), WHITE);

            for kite in 0..6 {
                if hex[h.0 as usize][h.1 as usize][kite]
                    != hex[h.0 as usize][h.1 as usize][(kite + 5) % 6]
                {
                    // draw line from center
                    let n = h.neighbor(kite);
                    let midpoint = avg(h.screen(), n.screen());
                    pict.draw_line(h.screen(), midpoint, YELLOW);
                }
            }
        }
    }

    /*

            // Only directions 2 and 3 as cell above took care of 0
            // and cells to the left took care of the rest
            let n2 = h.neighbor(2);

            if hex[h.0][h.1][1] != hex[n2.0][n2.1][5] {
            // Draw a separating line between the two kites



            let n3 = h.neighbor(3);

            for d in 2.
                // Plot top line
            // Neighbor center
                let nc: Vec<(isize, isize)> = (0..6).map(|d| h.neighbor(d).screen()).collect();
                let c = h.screen();
                let lefttop = avg3(c, nc[0], nc[5]);
                let righttop = avg3(cc, nc[0], nc[1]);

                pict.draw_line(lefttop, righttop, WHITE);
                //pict.draw_line(h.screen(), h0.screen(), WHITE);
            }
        }
    */

    pict.dump_iterm2_image(Some(100));

    /*
        for y in 1..N {
            for yy in 0..6usize {
                // 0.. -> 3,2,1,0,1,2,
                let spaces = if yy <= 3 { 3 - yy } else { yy - 3 };
                for _ in 0..spaces {
                    print!(" ");
                }

                for x in 0..N {
                    let e = if x % 2 == 0 { hex[y][x] } else { hex[y - 1][x] };

                    match (yy + (x % 2) * 3) % 6 {
                        0 => print!(">---"),
                        1 => print!("/{} | {}", e[5], e[0]),
                        2 => print!("/ \\ | / "),
                        3 => print!("<{}   +   {}", e[4], e[1]),
                        4 => print!("\\ / | \\ "),
                        5 => print!("\\{} | {}", e[3], e[2]),
                        x => panic!("{x} should have been impossible!"),
                    }
                }
                println!();
            }
        }
    */
}
