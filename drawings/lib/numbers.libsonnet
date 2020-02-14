// thin numbers library
{
  '2':
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
  '5': { type: 'sequence', objects: [
    {
      type: 'line',
      a: { x: -25, y: -40 },
      b: { x: 25, y: -40 },
    },
    {
      type: 'line',
      a: { x: -25, y: -40 },
      b: { x: -25, y: -10 },
    },
    {
      type: 'spline',
      a: { x: -25, y: -10 },
      sa: { x: 40, y: -10 },
      b: { x: -25, y: 40 },
      sb: { x: 40, y: 40 },

    },
  ] },
  '-': { type: 'sequence', objects: [
    {
      type: 'line',
      a: { x: -25, y: 0 },
      b: { x: 25, y: 0 },
    },
  ] },
  '0': { type: 'sequence', objects: [
    {
      type: 'spline',
      a: { x: 0, y: -40 },
      sa: { x: -25, y: -40 },
      b: { x: -25, y: 0 },
      sb: { x: -25, y: -20 },
    },
    {
      type: 'spline',
      a: { x: 0, y: -40 },
      sa: { x: 25, y: -40 },
      b: { x: 25, y: 0 },
      sb: { x: 25, y: -20 },
    },

    {
      type: 'spline',
      a: { x: 0, y: 40 },
      sa: { x: -25, y: 40 },
      b: { x: -25, y: 0 },
      sb: { x: -25, y: 20 },
    },
    {
      type: 'spline',
      a: { x: 0, y: 40 },
      sa: { x: 25, y: 40 },
      b: { x: 25, y: 0 },
      sb: { x: 25, y: 20 },
    },
  ] },
}
