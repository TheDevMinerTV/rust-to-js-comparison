import { empty_fn as emptyFn, mult_mat4 as multMat4 } from "../bindings/bindings.ts";
import { createRequire } from "https://deno.land/std@0.112.0/node/module.ts";

const require = createRequire(import.meta.url);
const { calcAvg, mocks } = require("../lib/utils.js");

const mockedMultMat4 = multMat4.bind(
    null,
    { d: mocks.matrices.m1 },
    { d: mocks.matrices.m2 }
);

console.log(`emptyFn(): ${calcAvg(emptyFn)}ms`);
console.log(`mult_mat4(): ${calcAvg(mockedMultMat4)}ms`);
