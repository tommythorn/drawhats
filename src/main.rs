/*!

![Four hat tiling](four-hats.png)

# Drawing the "Hat" tiling

I'm fashinated by [the Hat](https://en.wikipedia.org/wiki/Einstein_problem)

Here's my from-1st-principle attempt at solving it.

Note, the code is atrociously bad, but optimization is straight
forward (?) once the code is working.  Currently the search is really
unacceptible naive.

The first problem is how to represent the world.  What I chose for
now, is to tile the plane with hexagons, subdivided into kites (the
Hat consists of eight connected kites).  The hexagon tiling is using
standard grid mapping where odd rows are offset by half a hexagon
(there are many other options).

About 90% of the time on this code was figuring out how to print them.
Currently the graphics only works if you launch this from an iTerm2
terminal.  That's trivial to fix.




# Find Hat tilings

TL;DR: an astonishing breakthough in irregular tilings occurred in 2022 [1,2,3] when David Smith
found the "Hat".  In this crate I aim to tile rectangles with the Hat, from *1st principle*.

- [1] [Up and Atom's entertaining coverage](https://www.youtube.com/watch?v=A1BhOVW8qZU)
- [2] [Wikipedia entry](https://en.wikipedia.org/wiki/Einstein_problem)
- [3] [Sci. Am. has a great story on this](https://www.scientificamerican.com/article/newfound-mathematical-einstein-shape-creates-a-never-repeating-pattern/)

The Hat is made up of eight "kites", where a kite is the six part of a hexagon you get from
dividing with the three lines that intersect the center and midpoints of the sides.  Thus if we
wish to tile our rectangle ("plane") with Hats then must first find a way tile it with kites.
Lacking better ideas, I tile my rectangle with hexagons and refer to the kites within by orientation
(0 being the 1 O'clock kites, 1, being the 3 O'olock etc.)

First, map hexagons to a grid

 ```
      >---<         >---<         >
     /     \       /     \       /
    /  0 0  \     /  2 0  \     /
   <    +    >---<    +    >---<
    \       /     \       /     \
     \     /  1 0  \     /  3 0  \
      >---<    +    >---<    +    >
     /     \       /     \       /
    /  0 1  \     /  2 1  \     /
   <    +    >---<    +    >---<
    \       /     \       /     \
     \     /  1 1  \     /  3 1  \
      >---<    +    >---<    +    >
     /     \       /     \       /
    /  0 2  \     /  2 2  \     /
   <    +    >---<    +    >---<
    \       /     \       /     \
     \     /  1 2  \     /  3 2  \
      >---<    +    >---<    +    >
 ```
which gives rise to a funny neigbor offsets:
- even columns: (0,-1), (1, -1), (1, 0), (0,1), (-1, 0), (-1, -1)
- odd columns:  (0,-1), (1,  0), (1, 1), (0,1), (-1, 1), (-1,  0)
*/

type KiteColor = u8;
const EMPTY: KiteColor = 0;

// Half the width of a display hexagon
const RADIUS: isize = 32;

#[derive(Copy, Clone, PartialEq, Debug)]
struct HexPos(isize, isize);

impl HexPos {
    fn display_center(self) -> (isize, isize) {
        let y = RADIUS * (self.1 * 2 + 1 + self.0 % 2);
        let x = RADIUS * (self.0 * 2 + 1) * 866 / 1000; // 2cos(30)

        (x, y)
    }

    fn neighbor(self, direction: usize) -> HexPos {
        let HexPos(x, y) = self;
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

        HexPos(x + xd, y + yd)
    }
}

mod test {
    use super::*;
    #[allow(dead_code)]
    fn all_neighbors(h: (isize, isize)) -> Vec<(isize, isize)> {
        (0..6)
            .map(|d| {
                let n = HexPos(h.0, h.1).neighbor(d);
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

struct KiteGrid<const N: usize> {
    // Note, rounded 6 up to 8 because alignment is nice.
    hex: [[[KiteColor; 8]; N]; N],
}

impl<const N: usize> KiteGrid<N> {
    fn valid(&self, h: HexPos) -> bool {
        let HexPos(x, y) = h;
        (0..N as isize).contains(&x) && (0..N as isize).contains(&y)
    }

    fn get(&self, h: HexPos, kite: usize) -> KiteColor {
        let HexPos(x, y) = h;
        self.hex[x as usize][y as usize][kite]
    }

    fn set(&mut self, h: HexPos, kite: usize, c: KiteColor) {
        let HexPos(x, y) = h;
        self.hex[x as usize][y as usize][kite] = c;
    }

    fn hat_is_empty(&self, h: HexPos, kite: usize) -> bool {
        let n1 = h.neighbor((kite + 1) % 6);
        let n2 = h.neighbor((kite + 2) % 6);

        self.get(h, kite) == EMPTY
            && self.get(h, (kite + 1) % 6) == EMPTY
            && self.get(n1, (kite + 2) % 6) == EMPTY
            && self.get(n1, (kite + 3) % 6) == EMPTY
            && self.get(n2, kite) == EMPTY
            && self.get(n2, (kite + 3) % 6) == EMPTY
            && self.get(n2, (kite + 4) % 6) == EMPTY
            && self.get(n2, (kite + 5) % 6) == EMPTY
    }

    fn hat_set(&mut self, h: HexPos, kite: usize, c: KiteColor) {
        let n1 = h.neighbor((kite + 1) % 6);
        let n2 = h.neighbor((kite + 2) % 6);

        self.set(h, kite, c);
        self.set(h, (kite + 1) % 6, c);
        self.set(n1, (kite + 2) % 6, c);
        self.set(n1, (kite + 3) % 6, c);
        self.set(n2, kite, c);
        self.set(n2, (kite + 3) % 6, c);
        self.set(n2, (kite + 4) % 6, c);
        self.set(n2, (kite + 5) % 6, c);
    }

    fn try_position(&mut self, h: HexPos, kite: usize, budget: u8) {
        if self.hat_is_empty(h, kite) {
            self.hat_set(h, kite, budget);
            self.search(budget - 1);
            self.hat_set(h, kite, EMPTY);
        }
    }

    fn search(&mut self, budget: u8) {
        if budget == 0 {
            /*
            for y in 2..N as isize - 2 {
                for x in 2..N as isize - 2 {
                    let h = HexPos(x, y);

                    for k in 0..6 {
                        if self.get(h, k) == EMPTY {
                            return;
                        }
                    }
                }
            }
            */

            //print!("{}[H", 27 as char);
            self.display_tiling();
            return;
        }

        for y in 2..N as isize - 2 {
            for x in 2..N as isize - 2 {
                for d in 0..6 {
                    self.try_position(HexPos(x, y), d, budget);
                }
            }
        }
    }

    fn new() -> Self {
        Self {
            hex: [[[EMPTY; 8]; N]; N],
        }
    }

    fn display_tiling(&self) {
        use iterm2canvas::Pict;
        const WHITE: u32 = 0xFFFFFF;
        const GREY: u32 = 0x7F7F7F;
        const YELLOW: u32 = 0xFFFF00;

        let mut pict = Pict::new(
            N as isize * 2 * RADIUS * 866 / 1000,
            N as isize * 2 * RADIUS,
        );

        // I just need the center point of hexagon h
        // The midpoint between the neighbor h' and h AVG(h',h) is the middle of the line between them.
        // Call the left neighbor of h and h' L(h,h') and R(h,h') then the avg of h, h', and L(

        for y in 0..N as isize {
            for x in 0..N as isize {
                let h = HexPos(x, y);

                pict.plot(h.display_center(), WHITE);
                let center = h.display_center();

                for kite in 0..6 {
                    // Interiour lines
                    // draw line from center
                    let n = h.neighbor(kite);
                    let nn = h.neighbor((kite + 1) % 6);
                    let corner = avg3(center, n.display_center(), nn.display_center());
                    let midpoint = avg(center, n.display_center());

                    // XXX Imagine a brilliant kite painting code
                    // drawing the center-corner-midpoint triangle
                    // instead of this
                    //pict.draw_triangle(center,corner,midpoint, GREY);
                    if self.get(h, kite) != EMPTY {
                        let kite_center = avg3(center, corner, midpoint);
                        for x in -3..=3 {
                            for y in -3..=3 {
                                pict.plot((kite_center.0 + x, kite_center.1 + y), GREY);
                            }
                        }
                    }

                    if self.get(h, kite) != self.get(h, (kite + 5) % 6) {
                        // draw line from center
                        pict.draw_line(center, midpoint, YELLOW);
                    }

                    // Exterior lines
                    if self.valid(n) && self.get(h, kite) != self.get(n, (kite + 2) % 6) {
                        pict.draw_line(midpoint, corner, YELLOW);
                    }
                }
            }
        }

        pict.dump_iterm2_image(None);
    }
}

fn main() {
    /*
    for o in 0..6 {
        let mut kitegrid = KiteGrid::<6>::new();
        kitegrid.hat_set(HexPos(2, 2), o, 1);
        kitegrid.hat_set(HexPos(3, 3), 1, 2);
        kitegrid.display_tiling();
    }
    */

    let mut kitegrid = KiteGrid::<8>::new();
    for budget in 8..=255 {
        println!("Budget {budget}");
        kitegrid.search(budget);
    }
}
