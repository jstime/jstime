// Mid-level module X - depends on util-a and util-b
import { processA } from './util-a.js';
import { processB } from './util-b.js';

export function libX(value) {
    return processA(processB(value));
}
