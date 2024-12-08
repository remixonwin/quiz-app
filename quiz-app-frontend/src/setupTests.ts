// jest-dom adds custom jest matchers for asserting on DOM nodes.
// allows you to do things like:
// expect(element).toHaveTextContent(/react/i)
// learn more: https://github.com/testing-library/jest-dom
import '@testing-library/jest-dom';
import { configure } from '@testing-library/react';

// Configure Testing Library
configure({
  asyncUtilTimeout: 5000,
});

// Mock window.matchMedia
Object.defineProperty(window, 'matchMedia', {
  writable: true,
  value: jest.fn().mockImplementation(query => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: jest.fn(), // deprecated
    removeListener: jest.fn(), // deprecated
    addEventListener: jest.fn(),
    removeEventListener: jest.fn(),
    dispatchEvent: jest.fn(),
  })),
});

// Suppress act() warnings
const originalError = console.error;
console.error = (...args) => {
  if (/Warning: ReactDOM.render is no longer supported in React 18/.test(args[0])) {
    return;
  }
  if (/Warning: `ReactDOMTestUtils.act` is deprecated/.test(args[0])) {
    return;
  }
  if (/Warning: The current testing environment is not configured to support act/.test(args[0])) {
    return;
  }
  originalError.call(console, ...args);
};

// Increase Jest timeout
jest.setTimeout(10000);
