// create a composition
local composition(x, y, angle=0, size=100, query='head1') = {
  placement: {
    absolute: {
      x: x,
      y: y,
      angle: angle,
    },
  },
  size: size,
  query: {
    by_name: query,
  },
};

local icons = import 'icons.libsonnet';

local line(y, size=100, angle=0) =
  std.map(function(x) composition(x=x, y=y, size=size, angle=angle),
          std.map(function(x) x * size, std.range(-100, 100)));

{
  width: 600,
  height: 600,
  compositions: [
                  composition(x=300, y=300, query='x'),
                  //composition(x=100, y=100, query='hrule'),
                  //composition(x=100, y=200, query='hrule'),
                  //composition(x=100, y=300, query='hrule'),
                  //composition(x=100, y=400, query='hrule'),
                  //composition(x=100, y=500, query='hrule'),
                ]
                #+ line(y=50, size=100, angle=180)
                #+ line(y=150, size=100, angle=0)
                #+ line(y=250, size=100, angle=180)
                #+ line(y=350, size=100, angle=0)
                #+ line(y=450, size=100, angle=180)
                + line(y=550, size=100, angle=0),
  objects:
    { hrule: { type: 'line', a: { x: -1000, y: 0 }, b: { x: 10000, y: 0 } } }
    + { x:
      { type: 'sequence', objects: [
        { type: 'line', a: { x: -50, y: -50 }, b: { x: 50, y: 50 } },
        { type: 'line', a: { x: -50, y: 50 }, b: { x: 50, y: -50 } },
      ] } }
    + icons,
}
