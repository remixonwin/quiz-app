/// <reference types="cypress" />

describe('Quiz Application', () => {
  beforeEach(() => {
    // Intercept API calls
    cy.intercept('POST', 'http://localhost:8080/api/auth/login').as('login')
    cy.intercept('GET', 'http://localhost:8080/api/quizzes').as('getQuizzes')
    
    // Visit the home page and wait for it to load
    cy.visit('/')
    cy.window().its('document').should('exist')
    
    // Set a mock token
    cy.window().then((win) => {
      win.localStorage.setItem('token', 'mock-jwt-token')
    })
  })

  it('should load the home page', () => {
    cy.get('h1').should('exist')
  })

  it('should navigate to quiz page', () => {
    // Mock the quizzes response
    cy.intercept('GET', 'http://localhost:8080/api/quizzes', {
      statusCode: 200,
      body: [{
        id: 1,
        title: 'Test Quiz',
        description: 'A test quiz'
      }]
    }).as('getQuizzes')

    cy.get('[data-testid="start-quiz"]').should('be.visible').click()
    cy.url().should('include', '/quiz')
  })

  it('should be able to answer questions', () => {
    // Mock the quizzes and questions
    cy.intercept('GET', 'http://localhost:8080/api/quizzes', {
      statusCode: 200,
      body: [{
        id: 1,
        title: 'Test Quiz',
        description: 'A test quiz'
      }]
    }).as('getQuizzes')

    cy.intercept('GET', 'http://localhost:8080/api/quizzes/1', {
      statusCode: 200,
      body: {
        quiz: {
          id: 1,
          title: 'Test Quiz',
          description: 'A test quiz'
        },
        questions: [{
          id: 1,
          text: 'Test Question',
          options: ['Option 1', 'Option 2', 'Option 3', 'Option 4']
        }]
      }
    }).as('getQuizDetails')

    cy.get('[data-testid="start-quiz"]').should('be.visible').click()
    cy.get('[data-testid="quiz-question"]').should('be.visible')
    cy.get('[data-testid="answer-option"]').first().should('be.visible').click()
    cy.get('[data-testid="next-question"]').should('be.visible').click()
  })

  it('should show results at the end', () => {
    // Mock the necessary API responses
    cy.intercept('GET', 'http://localhost:8080/api/quizzes', {
      statusCode: 200,
      body: [{
        id: 1,
        title: 'Test Quiz',
        description: 'A test quiz'
      }]
    }).as('getQuizzes')

    cy.intercept('GET', 'http://localhost:8080/api/quizzes/1', {
      statusCode: 200,
      body: {
        quiz: {
          id: 1,
          title: 'Test Quiz',
          description: 'A test quiz'
        },
        questions: [{
          id: 1,
          text: 'Test Question',
          options: ['Option 1', 'Option 2', 'Option 3', 'Option 4']
        }]
      }
    }).as('getQuizDetails')

    cy.intercept('POST', 'http://localhost:8080/api/quizzes/1/submit', {
      statusCode: 200,
      body: {
        score: 80
      }
    }).as('submitQuiz')

    cy.get('[data-testid="start-quiz"]').should('be.visible').click()
    cy.get('[data-testid="answer-option"]').each(($option: JQuery<HTMLElement>) => {
      cy.wrap($option).should('be.visible').click()
      cy.get('[data-testid="next-question"]').should('be.visible').click()
    })
    cy.get('[data-testid="quiz-results"]').should('be.visible')
  })
})
