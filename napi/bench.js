const { calcAvg, mocks } = require("../lib/utils");
const { emptyFn, multMat4 } = require(".");

const mockedMultMat4 = multMat4.bind(
    null,
    mocks.matrices.m1,
    mocks.matrices.m2
);

console.log(`emptyFn(): ${calcAvg(emptyFn)}ms`);
console.log(`multMat4(): ${calcAvg(mockedMultMat4)}ms`);
