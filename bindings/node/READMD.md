# @initencounter/cube

魔方核心库

## example

```javascript
import {CubeCore} from './index.js';
import  {writeFileSync} from 'fs';

let cube_ = new CubeCore();
cube_.rotate('U');
cube_.rotate('U');
console.log(cube_.getCube());
console.log(cube_.getStartTime());
let base64 = cube_.getSvgBase64Png();
writeFileSync('./test.png', Buffer.from(base64.replace('data:image/png;base64,',''),'base64'));
```