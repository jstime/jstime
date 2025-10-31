// Mid-level module Y - depends on util-b and util-c
import { processB } from './util-b.js';
import { processC } from './util-c.js';

export function libY(value) {
    return processB(processC(value));
}
