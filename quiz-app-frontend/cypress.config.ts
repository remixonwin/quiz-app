const { defineConfig } = require('cypress')

module.exports = defineConfig({
  projectId: '3m8mi8',
  e2e: {
    baseUrl: 'http://localhost:3000',
    supportFile: 'cypress/support/e2e.ts',
    specPattern: 'cypress/e2e/**/*.cy.ts',
    video: false,
    screenshotOnRunFailure: true,
    setupNodeEvents(on, config) {
      return config
    }
  }
})
