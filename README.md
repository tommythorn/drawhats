# drawhats


![Four hat tiling](four-hats.png)

## Drawing the "Hat" tiling

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




## Find Hat tilings

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
