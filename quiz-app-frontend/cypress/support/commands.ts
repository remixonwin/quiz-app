import '@testing-library/cypress/add-commands'

declare global {
  namespace Cypress {
    interface Chainable {
      login(email: string, password: string): Chainable<void>
      setupTestEnv(): Chainable<void>
    }
  }
}

// You can add custom commands here if needed
// Login command
Cypress.Commands.add('login', (email: string, password: string) => {
  cy.visit('/login')
  cy.get('[data-testid="email-input"]').type(email)
  cy.get('[data-testid="password-input"]').type(password)
  cy.get('[data-testid="login-button"]').click()
})

// Setup test environment command
Cypress.Commands.add('setupTestEnv', () => {
  cy.intercept('GET', 'http://localhost:8080/api/quizzes', {
    statusCode: 200,
    body: [
      {
        id: 1,
        title: 'Test Quiz',
        description: 'A test quiz',
        questions: [
          {
            id: 1,
            text: 'Test Question',
            options: ['Option 1', 'Option 2', 'Option 3', 'Option 4'],
            correctAnswer: 0
          }
        ]
      }
    ]
  }).as('getQuizzes')

  cy.window().then((win) => {
    win.localStorage.setItem('token', 'test-token')
  })
})
