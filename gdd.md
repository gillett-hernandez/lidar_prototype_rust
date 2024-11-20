# Game Design document

or rather, some notes that i can't be bothered to put as comments in source code

src/space.rs :

- need to determine the best way to do the actual rendering of incredible amounts of points.
- obviously the easiest solution is to just spawn a mesh for each point, but that's inefficient and will quickly reach a performance limit
- another possibility is to have each chunk of an OctTree have a Cube mesh that runs a fragment shader that renders all the points within it based on a viewpoint.
  - at far distances / low LoDs, this could summarize its contents and keep performance high by just rendering as a constant color,
  - or by rendering the entire cubeoid as a point light with brightness set according to the number of points within the cuboid