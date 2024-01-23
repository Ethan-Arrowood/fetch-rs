import test from 'node:test';
import assert from 'node:assert/strict';
import { fetch, Request } from '../index.js';

const url = 'http://example.com'

test('fetch', t => {
    assert.doesNotReject(() => {
        return fetch(url)
    });
    assert.doesNotReject(() => {
        return fetch(url, {});
    });
    assert.doesNotReject(() => {
        return  fetch(new Request(url));
    });
    assert.doesNotReject(() => {
        return fetch(new Request(url), {});
    });
});

test('request', t => {
    const req = new Request(url);
    assert.strict(req.url.host, 'example.com')
})