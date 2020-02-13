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
}
