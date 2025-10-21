import { constants } from 'node:fs/promises';

globalThis.testConstants = 
  typeof constants.F_OK === 'number' &&
  typeof constants.R_OK === 'number' &&
  typeof constants.W_OK === 'number' &&
  typeof constants.X_OK === 'number';
