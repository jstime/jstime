// Event and EventTarget
// https://dom.spec.whatwg.org/#interface-event
// https://dom.spec.whatwg.org/#interface-eventtarget

'use strict';

// eslint-disable-next-line no-unused-expressions
(({ 
  eventTargetAddEventListener,
  eventTargetRemoveEventListener,
  eventTargetDispatchEvent,
  eventStopPropagation,
  eventStopImmediatePropagation,
  eventPreventDefault
}) => {
  
  // Event phases
  const NONE = 0;
  const CAPTURING_PHASE = 1;
  const AT_TARGET = 2;
  const BUBBLING_PHASE = 3;

  // Event class
  class Event {
    #type;
    #bubbles;
    #cancelable;
    #composed;
    #timeStamp;

    constructor(type, eventInitDict = {}) {
      this.#type = String(type);
      this.#bubbles = Boolean(eventInitDict.bubbles);
      this.#cancelable = Boolean(eventInitDict.cancelable);
      this.#composed = Boolean(eventInitDict.composed);
      this.#timeStamp = performance.now();
      
      // Internal state
      this.__target__ = null;
      this.__currentTarget__ = null;
      this.__eventPhase__ = NONE;
      this.__stopPropagation__ = false;
      this.__stopImmediatePropagation__ = false;
      this.__defaultPrevented__ = false;
      this.__isTrusted__ = false;
    }

    get type() {
      return this.#type;
    }

    get target() {
      return this.__target__;
    }

    get currentTarget() {
      return this.__currentTarget__;
    }

    get eventPhase() {
      return this.__eventPhase__ || NONE;
    }

    stopPropagation() {
      eventStopPropagation(this);
    }

    stopImmediatePropagation() {
      eventStopImmediatePropagation(this);
    }

    get bubbles() {
      return this.#bubbles;
    }

    get cancelable() {
      return this.#cancelable;
    }

    preventDefault() {
      eventPreventDefault(this);
    }

    get defaultPrevented() {
      return this.__defaultPrevented__ || false;
    }

    get composed() {
      return this.#composed;
    }

    get isTrusted() {
      return this.__isTrusted__ || false;
    }

    get timeStamp() {
      return this.#timeStamp;
    }

    // Constants
    static get NONE() { return NONE; }
    static get CAPTURING_PHASE() { return CAPTURING_PHASE; }
    static get AT_TARGET() { return AT_TARGET; }
    static get BUBBLING_PHASE() { return BUBBLING_PHASE; }

    get NONE() { return NONE; }
    get CAPTURING_PHASE() { return CAPTURING_PHASE; }
    get AT_TARGET() { return AT_TARGET; }
    get BUBBLING_PHASE() { return BUBBLING_PHASE; }
  }

  // EventTarget class
  class EventTarget {
    constructor() {
      // Internal storage for event listeners (managed by Rust side)
      this.__listeners__ = null;
    }

    addEventListener(type, listener, options = {}) {
      if (listener === null || listener === undefined) {
        return;
      }

      // Convert listener to a function if it has handleEvent method
      let callback = listener;
      if (typeof listener === 'object' && typeof listener.handleEvent === 'function') {
        callback = listener.handleEvent.bind(listener);
      }

      if (typeof callback !== 'function') {
        return;
      }

      // Call Rust implementation
      eventTargetAddEventListener(this, String(type), callback, options);
    }

    removeEventListener(type, listener, options = {}) {
      if (listener === null || listener === undefined) {
        return;
      }

      // Convert listener to a function if it has handleEvent method
      let callback = listener;
      if (typeof listener === 'object' && typeof listener.handleEvent === 'function') {
        callback = listener.handleEvent.bind(listener);
      }

      if (typeof callback !== 'function') {
        return;
      }

      // Call Rust implementation
      eventTargetRemoveEventListener(this, String(type), callback, options);
    }

    dispatchEvent(event) {
      if (!(event instanceof Event)) {
        throw new TypeError('Failed to execute \'dispatchEvent\' on \'EventTarget\': parameter 1 is not of type \'Event\'.');
      }

      // Call Rust implementation
      return eventTargetDispatchEvent(this, event);
    }
  }

  // Export to global scope
  globalThis.Event = Event;
  globalThis.EventTarget = EventTarget;
});
