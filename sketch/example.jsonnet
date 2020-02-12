// create a composition
local composition(x, y, size=100) = {
  placement: {
    absolute: {
      x: x,
      y: y,
    },
  },
  size: size,
  query: {
    icon: 'head1',
  },
};

local icons = import 'icons.libsonnet';

local line(y, size=100) =
  std.map(function(x) composition(x, y, size),
          std.map(function(x) x * size, std.range(-100, 100)));

{
  width: 600,
  height: 600,
  compositions: [
                ]
                + line(50, 100)
                + line(150, 100)
                + line(250, 100),
  icons: icons,
}
