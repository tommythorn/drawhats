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
const GREY: u32 = 0x7F7F7F;
const YELLOW: u32 = 0xFFFF00;

const N: isize = 7;

const RADIUS: isize = 32;

#[derive(Copy, Clone, PartialEq, Debug)]
struct Hex(isize, isize);

type KiteColor = usize;
const EMPTY: KiteColor = 0;

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

fn avg3(p1: (isize, isize), p2: (isize, isize), p3: (isize, isize)) -> (isize, isize) {
    ((p1.0 + p2.0 + p3.0) / 3, (p1.1 + p2.1 + p3.1) / 3)
}

fn plot_hex(hex: &[[[KiteColor; 6]; N as usize]; N as usize]) {
    let mut pict = Pict::new((N * 2 + 1) * RADIUS, (N * 2 + 1) * RADIUS);

    // I just need the center point of hexagon h
    // The midpoint between the neighbor h' and h AVG(h',h) is the middle of the line between them.
    // Call the left neighbor of h and h' L(h,h') and R(h,h') then the avg of h, h', and L(

    for y in 0..N {
        for x in 0..N {
            let h = Hex(x, y);

            pict.plot(h.screen(), WHITE);
            let center = h.screen();

            for kite in 0..6 {
                // Interiour lines
                // draw line from center
                let n = h.neighbor(kite);
                let midpoint = avg(center, n.screen());
                let nn = h.neighbor((kite + 1) % 6);
                let corner = avg3(center, n.screen(), nn.screen());

                // XXX Imagine a brilliant kite painting code
                // drawing the center-corner-midpoint triangle
                // instead of this
                //pict.draw_triangle(center,corner,midpoint, GREY);
                if hex[h.0 as usize][h.1 as usize][kite] != EMPTY {
                    let kite_center = avg3(center, corner, midpoint);
                    {
                        for x in -3..=3 {
                            for y in -3..=3 {
                                pict.plot((kite_center.0 + x, kite_center.1 + y), GREY);
                            }
                        }
                    }
                }

                if hex[h.0 as usize][h.1 as usize][kite]
                    != hex[h.0 as usize][h.1 as usize][(kite + 5) % 6]
                {
                    // draw line from center
                    let n = h.neighbor(kite);
                    let midpoint = avg(center, n.screen());
                    pict.draw_line(center, midpoint, YELLOW);
                }

                // Exterior lines
                let n = h.neighbor(kite);
                if 0 <= n.0
                    && n.0 < N
                    && 0 <= n.1
                    && n.1 < N
                    && hex[h.0 as usize][h.1 as usize][kite]
                        != hex[n.0 as usize][n.1 as usize][(kite + 2) % 6]
                {
                    pict.draw_line(midpoint, corner, YELLOW);
                }
            }
        }
    }

    pict.dump_iterm2_image(Some(100));
}

fn get(hex: &mut [[[KiteColor; 6]; N as usize]; N as usize], h: Hex, kite: usize) -> KiteColor {
    let Hex(x, y) = h;

    if (0..N).contains(&x) && (0..N).contains(&y) {
        hex[x as usize][y as usize][kite]
    } else {
        !0 // Some non-empty color
    }
}

fn set(hex: &mut [[[KiteColor; 6]; N as usize]; N as usize], h: Hex, kite: usize, c: KiteColor) {
    let Hex(x, y) = h;
    if (0..N).contains(&x) && (0..N).contains(&y) {
        hex[x as usize][y as usize][kite] = c;
    } else {
        panic!("set({h:?})");
    }
}

fn hat_is_empty(hex: &mut [[[KiteColor; 6]; N as usize]; N as usize], h: Hex, kite: usize) -> bool {
    let n1 = h.neighbor((kite + 1) % 6);
    let n2 = h.neighbor((kite + 2) % 6);

    get(hex, h, kite) == EMPTY
        && get(hex, h, (kite + 1) % 6) == EMPTY
        && get(hex, n1, (kite + 2) % 6) == EMPTY
        && get(hex, n1, (kite + 3) % 6) == EMPTY
        && get(hex, n2, kite) == EMPTY
        && get(hex, n2, (kite + 3) % 6) == EMPTY
        && get(hex, n2, (kite + 4) % 6) == EMPTY
        && get(hex, n2, (kite + 5) % 6) == EMPTY
}

fn hat_set(
    hex: &mut [[[KiteColor; 6]; N as usize]; N as usize],
    h: Hex,
    kite: usize,
    c: KiteColor,
) {
    let n1 = h.neighbor((kite + 1) % 6);
    let n2 = h.neighbor((kite + 2) % 6);

    set(hex, h, kite, c);
    set(hex, h, (kite + 1) % 6, c);
    set(hex, n1, (kite + 2) % 6, c);
    set(hex, n1, (kite + 3) % 6, c);
    set(hex, n2, kite, c);
    set(hex, n2, (kite + 3) % 6, c);
    set(hex, n2, (kite + 4) % 6, c);
    set(hex, n2, (kite + 5) % 6, c);
}

fn try_position(
    hex: &mut [[[KiteColor; 6]; N as usize]; N as usize],
    h: Hex,
    kite: usize,
    budget: usize,
) {
    if hat_is_empty(hex, h, kite) {
        hat_set(hex, h, kite, budget);

        search(hex, budget - 1);

        hat_set(hex, h, kite, EMPTY);
    }
}

fn search(hex: &mut [[[KiteColor; 6]; N as usize]; N as usize], budget: usize) {
    if budget == 0 {
        print!("{}[H", 27 as char);
        plot_hex(hex);
        return;
    }

    for y in 2..N - 2 {
        for x in 2..N - 2 {
            for d in 0..6 {
                try_position(hex, Hex(x, y), d, budget);
            }
        }
    }
}

fn main() {
    let mut hex = [[[EMPTY; 6]; N as usize]; N as usize];

    /*
        // Highlight a hat
        hex[2][2][1] = '*';
        hex[2][2][2] = '*';
        hex[3][2][4] = '*';
        hex[3][2][3] = '*';
        hex[2][3][0] = '*';
        hex[2][3][1] = '*';
        hex[2][3][4] = '*';
        hex[2][3][5] = '*';
    */

    search(&mut hex, 11);

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
