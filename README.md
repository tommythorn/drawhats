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

