describe('Quiz App', () => {
  beforeEach(() => {
    cy.visit('/')
  })

  it('should display the home page', () => {
    cy.get('[data-testid="home"]').should('exist')
  })

  it('should start a quiz', () => {
    cy.get('[data-testid="start-quiz"]').click()
    cy.get('[data-testid="quiz-take"]').should('exist')
  })

  it('should complete a quiz and show results', () => {
    cy.get('[data-testid="start-quiz"]').click()
    cy.get('[data-testid="quiz-take"]').should('exist')
    
    // Answer all questions (assuming there are questions)
    cy.get('[data-testid="answer-option"]').first().click()
    cy.get('[data-testid="next-question"]').click()
    
    // Should show results at the end
    cy.get('[data-testid="quiz-results"]').should('exist')
  })
})
