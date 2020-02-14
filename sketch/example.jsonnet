// example usecase:
// ----------------
local icon_setup = import './icon-studio.libsonnet';

icon_setup.icon_setup(
  query=
  { type: 'sequence', objects: [
    { type: 'circle' },
    {
      type: 'ring',
      radius: 35,
      color: 'background',
    },
    {
      type: 'circle',
      radius: 20,
      color: 'background',
    },
    {
      type: 'sequence',
      objects: [
        {
          type: 'line',
          a: { y: -50 },
          b: { y: 50 },
          color: 'background',
        },
        {
          type: 'line',
          a: { x: -50 },
          b: { x: 50 },
          color: 'background',
        },
      ],
    },
    {
      type: 'sequence',
      angle: 45,
      objects: [
        {
          type: 'line',
          a: { y: -50 },
          b: { y: 50 },
          color: 'background',
        },
        {
          type: 'line',
          a: { x: -50 },
          b: { x: 50 },
          color: 'background',
        },
      ],
    },
    { type: 'circle', radius: 10 },
  ] },
)
