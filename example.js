import { fetch } from './index.js'

fetch('foo');
fetch('foo', {});
fetch(new Request('localhost:3000'));
fetch(new Request('localhost:3000'), {});