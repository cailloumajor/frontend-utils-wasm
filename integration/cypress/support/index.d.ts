/// <reference types="cypress" />

declare namespace Cypress {
  interface Chainable {
    dataCy<E extends Node = HTMLElement>(value: string): Chainable<JQuery<E>>
  }
}
