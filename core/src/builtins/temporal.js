// Temporal API
// https://tc39.es/proposal-temporal/

'use strict';

// eslint-disable-next-line no-unused-expressions
(({ temporalNow, temporalPlainDate, temporalPlainTime, temporalPlainDateTime, temporalInstant }) => {
  // Create the Temporal namespace
  globalThis.Temporal = {
    Now: {
      instant() {
        return temporalNow('instant');
      },
      plainDateISO() {
        return temporalNow('plainDateISO');
      },
      plainTimeISO() {
        return temporalNow('plainTimeISO');
      },
      plainDateTimeISO() {
        return temporalNow('plainDateTimeISO');
      },
      zonedDateTimeISO() {
        return temporalNow('zonedDateTimeISO');
      },
    },
    PlainDate: function(year, month, day) {
      return temporalPlainDate(year, month, day);
    },
    PlainTime: function(hour, minute, second, millisecond, microsecond, nanosecond) {
      return temporalPlainTime(hour || 0, minute || 0, second || 0, millisecond || 0, microsecond || 0, nanosecond || 0);
    },
    PlainDateTime: function(year, month, day, hour, minute, second, millisecond, microsecond, nanosecond) {
      return temporalPlainDateTime(year, month, day, hour || 0, minute || 0, second || 0, millisecond || 0, microsecond || 0, nanosecond || 0);
    },
    Instant: {
      from(item) {
        return temporalInstant(item);
      }
    }
  };
});
