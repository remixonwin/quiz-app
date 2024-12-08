import React from 'react';
import { render, RenderOptions } from '@testing-library/react';
import { BrowserRouter, createBrowserRouter, RouterProvider, createRoutesFromElements, Route } from 'react-router-dom';
import { act } from 'react';

interface WrapperProps {
    children: React.ReactNode;
}

const AllTheProviders = ({ children }: WrapperProps) => {
    return (
        <BrowserRouter>
            {children}
        </BrowserRouter>
    );
};

const customRender = async (
    ui: React.ReactElement,
    options?: Omit<RenderOptions, 'wrapper'>
) => {
    let result: ReturnType<typeof render> | undefined;
    await act(async () => {
        result = render(ui, { wrapper: AllTheProviders, ...options });
    });
    return result!;
};

// re-export everything
export * from '@testing-library/react';

// override render method
export { customRender as renderWithRouter };

// Wrapper with BrowserRouter for simpler tests
export const renderWithRouterSimple = (ui: React.ReactElement, { route = '/' } = {}) => {
  window.history.pushState({}, 'Test page', route);
  
  return render(
    <BrowserRouter>
      {ui}
    </BrowserRouter>
  );
};

// For more complex routing scenarios
export const renderWithRouterConfig = (ui: React.ReactElement, { route = '/' } = {}) => {
  window.history.pushState({}, 'Test page', route);
  
  const router = createBrowserRouter(
    createRoutesFromElements(
      <Route path="/" element={ui} />
    )
  );
  
  return render(<RouterProvider router={router} />);
};
