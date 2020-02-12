local width = 1366;
local height = 768;

// create a composition
local composition(x=0, y=0, angle=0, size=100, query='head1') = {
  type: 'placement',
  x: x,
  y: y,
  angle: angle,
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
  width: width,
  height: height,
  start: { by_name: 'center' },
  objects:
    {
      hrule: { type: 'line', a: { x: -1000, y: 0 }, b: { x: 10000, y: 0 } },
    } + {
      center:
        {
          type: 'placement',
          x: width / 2,
          y: height / 2,
          size: 100,
          query: { by_name: 'main' },
        },
    } + {
      x:
        { type: 'sequence', objects: [
          { type: 'line', a: { x: -50, y: -50 }, b: { x: 50, y: 50 } },
          { type: 'line', a: { x: -50, y: 50 }, b: { x: 50, y: -50 } },
        ] },
    } + {
      hline: {
        type: 'sequence',
        objects: [
          composition(y=-30, size=20, query='harrow2'),
          composition(y=-10, size=20, query='harrow'),
          composition(y=10, size=20, query='harrow2'),
          composition(y=30, size=20, query='harrow'),
        ],
      },
    } + {
      main: {
        type: 'sequence',
        objects: [
          composition(y=5 * 80, query='hline'),
          composition(y=4 * 80, query='hline'),
          composition(y=3 * 80, query='hline'),
          composition(y=2 * 80, query='hline'),
          composition(y=1 * 80, query='hline'),
          composition(y=0 * 80, query='hline'),
          composition(y=-5 * 80, query='hline'),
          composition(y=-4 * 80, query='hline'),
          composition(y=-3 * 80, query='hline'),
          composition(y=-2 * 80, query='hline'),
          composition(y=-1 * 80, query='hline'),
        ],
      },
    } + {
      harrow: {
        type: 'sequence',
        objects: line(y=0, size=100),
      },
    } + {
      harrow2: {
        type: 'sequence',
        objects: line(y=0, size=100, angle=180),
      },
    } + icons,
}
