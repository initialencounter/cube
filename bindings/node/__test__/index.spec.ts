import test from 'ava'

import { CubeCore } from '../index.js'

test('cube from native', (t) => {
  let operation = 'Uu';
  let cube = new CubeCore();
  cube.rotate(operation);
  t.is(operation, cube.getLastStep());
})
