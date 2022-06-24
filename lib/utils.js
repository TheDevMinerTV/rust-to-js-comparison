module.exports.calcAvg = (f) => {
  const times = [];

  for (let i = 0; i < 1_000_000; i++) {
    const start = performance.now();
    f();
    times.push(performance.now() - start);
  }

  return times.reduce((a, b) => a + b, 0) / times.length;
};

module.exports.mocks = {
  matrices: {
    m1: [
      [1, 2, 4, 8],
      [2, 4, 8, 16],
      [4, 8, 16, 32],
      [8, 16, 32, 64]
    ],
    m2: [
      [2, 4, 8, 16],
      [4, 8, 16, 32],
      [8, 16, 32, 64],
      [16, 32, 64, 128]
    ]
  }
};
