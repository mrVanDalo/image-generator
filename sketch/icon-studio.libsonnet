// a setup helping you defining your own icons
// and see them in different situations
//
// example usecase:
// ----------------
// local icon_setup = import './icon-setup.libsonnet';
//
// icon_setup.icon_setup(
//   query=
//   { type: 'sequence', objects: [
//     { type: 'line', a: { x: -50, y: 50 }, b: { x: 50, y: -50 } },
//     { type: 'line', a: { x: -50, y: -50 }, b: { x: 50, y: 50 } },
//   ] },
// )


local placement(x=0, y=0, angle=0, size=100, query='item') = {
  type: 'placement',
  x: x,
  y: y,
  angle: angle,
  size: size,
  query: {
    by_name: query,
  },
};

{
  icon_setup(width=1366, height=768, query)::
    {
      width: width,
      height: height,
      start: { by_name: 'center' },
      objects:
        {
          center:
            {
              type: 'placement',
              x: width / 2 - 200,
              y: height / 2,
              size: 100,
              query: { by_name: 'main' },
            },
          main: {
            type: 'sequence',
            objects: [
              // corners
              { type: 'line', a: { x: -60, y: -50 }, b: { x: -50, y: -50 } },
              { type: 'line', a: { x: -50, y: -60 }, b: { x: -50, y: -50 } },
              { type: 'line', a: { x: 60, y: -50 }, b: { x: 50, y: -50 } },
              { type: 'line', a: { x: 50, y: -60 }, b: { x: 50, y: -50 } },
              { type: 'line', a: { x: 50, y: 60 }, b: { x: 50, y: 50 } },
              { type: 'line', a: { x: 60, y: 50 }, b: { x: 50, y: 50 } },
              { type: 'line', a: { x: -50, y: 60 }, b: { x: -50, y: 50 } },
              { type: 'line', a: { x: -60, y: 50 }, b: { x: -50, y: 50 } },

              // middle
              { type: 'line', a: { x: -50, y: 0 }, b: { x: -80, y: -0 } },
              { type: 'line', a: { x: 50, y: 0 }, b: { x: 80, y: -0 } },
              { type: 'line', a: { x: 0, y: 50 }, b: { x: 0, y: 80 } },
              { type: 'line', a: { x: 0, y: -50 }, b: { x: 0, y: -80 } },

              // 25%
              { type: 'line', a: { x: -50, y: 25 }, b: { x: -70, y: 25 } },
              { type: 'line', a: { x: 50, y: 25 }, b: { x: 70, y: 25 } },
              { type: 'line', a: { y: -50, x: 25 }, b: { y: -70, x: 25 } },
              { type: 'line', a: { y: 50, x: 25 }, b: { y: 70, x: 25 } },

              // -25 %
              { type: 'line', a: { x: -50, y: -25 }, b: { x: -70, y: -25 } },
              { type: 'line', a: { x: 50, y: -25 }, b: { x: 70, y: -25 } },
              { type: 'line', a: { y: -50, x: -25 }, b: { y: -70, x: -25 } },
              { type: 'line', a: { y: 50, x: -25 }, b: { y: 70, x: -25 } },

              placement(query='item'),

              // hline
              placement(x=0, y=-200, query='item'),
              placement(x=-100, y=-200, query='item'),
              placement(x=-200, y=-200, query='item'),
              placement(x=-300, y=-200, query='item'),
              placement(x=-400, y=-200, query='item'),
              placement(x=-500, y=-200, query='item'),
              placement(x=100, y=-200, query='item'),
              placement(x=200, y=-200, query='item'),
              placement(x=300, y=-200, query='item'),
              placement(x=400, y=-200, query='item'),
              placement(x=500, y=-200, query='item'),
              placement(x=600, y=-200, query='item'),
              placement(x=700, y=-200, query='item'),

              // kacheln
              placement(x=300, y=0, query='item'),
              placement(x=400, y=0, query='item'),
              placement(x=500, y=0, query='item'),
              placement(x=600, y=0, query='item'),
              placement(x=700, y=0, query='item'),
              placement(x=300, y=100, query='item'),
              placement(x=400, y=100, query='item'),
              placement(x=500, y=100, query='item'),
              placement(x=600, y=100, query='item'),
              placement(x=700, y=100, query='item'),
              placement(x=300, y=200, query='item'),
              placement(x=400, y=200, query='item'),
              placement(x=500, y=200, query='item'),
              placement(x=600, y=200, query='item'),
              placement(x=700, y=200, query='item'),
              placement(x=300, y=300, query='item'),
              placement(x=400, y=300, query='item'),
              placement(x=500, y=300, query='item'),
              placement(x=600, y=300, query='item'),
              placement(x=700, y=300, query='item'),
            ],
          },
          item: query,
        },
    },
}
