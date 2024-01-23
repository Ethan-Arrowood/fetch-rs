import test from 'node:test';
import assert from 'node:assert/strict';
import { fetch } from '../index.js';

test('fetch', t => {
    assert.doesNotReject(() => {
        return fetch('foo')
    });
    assert.doesNotReject(() => {
        return fetch('foo', {});
    });
    assert.doesNotReject(() => {
        return  fetch(new Request('localhost:3000'));
    });
    assert.doesNotReject(() => {
        return fetch(new Request('localhost:3000'), {});
    });
});