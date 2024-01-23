import { fetchWithStr, fetchWithRequest, fetchWithStrAndInit, fetchWithRequestAndInit, Request } from './fetch-rs.cjs';

export function fetch (input, init) {
  if (typeof input === 'string') {
    if (init && typeof init === 'object') {
      return fetchWithStrAndInit(input, init);
    }
    return fetchWithStr(input);
  } else if (input instanceof Request) {
    if (init && typeof init === 'object') {
      return fetchWithRequestAndInit(input, init);
    }
    return fetchWithRequest(input);
  }
}

export { Request } from './fetch-rs.cjs'