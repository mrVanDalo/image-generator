local width = 1366;
local height = 768;


local icon_studio = import './icon-studio.libsonnet';
icon_studio.icon_setup(
  query=
  { type: 'sequence', objects: [
    {
      type: 'spline',
      a: { x: -25, y: -25 },
      b: { x: 25, y: -25 },
      sa: { x: -25, y: -50 },
      sb: { x: 25, y: -50 },
    },
    {
      type: 'spline',
      a: { x: 25, y: -25 },
      b: { x: -25, y: 40 },
      sa: { x: 25, y: 10 },
      sb: { x: -25, y: 20 },
    },
    {
      type: 'line',
      a: { x: -25, y: 40 },
      b: { x: 25, y: 40 },
    },

  ] },
)
