local width = 1366;
local height = 768;


local icon_studio = import './icon-studio.libsonnet';
icon_studio.icon_setup(
  query=
  { type: 'sequence', objects: [
    { type: 'line', a: { x: -50, y: 50 }, b: { x: 50, y: -50 } },
    { type: 'line', a: { x: -50, y: -50 }, b: { x: 50, y: 50 } },
  ] },
)
