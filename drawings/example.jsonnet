local line(y=0, query) =
  std.map(function(x)
    { type: 'placement', x: x * 100, y: y, query: query }
          , std.range(-10, 10));

local grid(query) =
  {
    type: 'sequence',
    objects: std.flattenArrays(std.map(function(y) line(
      y=100 * y,
      query=query
    ), std.range(-10, 10))),
  };


{
  width: 1366,
  height: 768,
  start: { by_name: 'main' },

  objects: {
    main: {
      type: 'sequence',
      objects: [
      { type: 'placement', query: { by_name: 'background_grid' } },
      { type: 'placement', query: { by_name: 'filling_grid' } },
      { type: 'placement', x: 50, y: 50, query: { by_name: 'filling_grid' } }
      ],
    },

    filling_grid: grid(query={ by_tag: ['filling'] }),
    eye: {
      type: 'sequence',
      tags: ['filling'],
      objects: [
        { type: 'ring', radius: 25 },
      ],
    },
    cross: {
      type: 'sequence',
      tags: ['filling'],
      objects: [
        {
          type: 'line',
          a: { y: -30 },
          b: { y: 30 },
        },
        {
          type: 'line',
          a: { x: -30 },
          b: { x: 30 },
        },
      ],
    },

    // background
    background_grid: grid(query={ by_name: 'background' }),
    background: { type: 'sequence', objects: [
      {
        type: 'line',
        a: { y: -50 },
        b: { x: -50 },
      },
      {
        type: 'line',
        a: { y: 50 },
        b: { x: 50 },
      },

      {
        type: 'line',
        a: { y: 50 },
        b: { x: -50 },
      },
      {
        type: 'line',
        a: { y: -50 },
        b: { x: 50 },
      },
    ] },
  },
}
