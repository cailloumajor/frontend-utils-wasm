describe("timeline", () => {
  beforeEach(() => {
    cy.visit("/")
    cy.window()
      .its("console")
      .then((console) => {
        cy.stub(console, "error").throwsArg(0)
      })
    cy.dataCy("timeline-canvas").as("canvas")
    cy.dataCy("draw-button").as("button").should("have.class", "ready")
    cy.dataCy("error-out").as("error").should("be.empty")
  })

  it("fails with an error (without throwing) if canvas was deleted", () => {
    cy.get<HTMLCanvasElement>("@canvas").invoke("remove")

    cy.get("@button").click()

    cy.get("@error").should(
      "include.text",
      "error parsing canvas style property `color`"
    )
  })

  it("fails if MessagePack deserialization errors", () => {
    cy.intercept("/timeline", { body: new Uint8Array([0xc1]).buffer })

    cy.get("@button").click()

    cy.get("@error")
      .should("include.text", "MessagePack")
      .and("include.text", "Reserved")
  })

  it("fails if there is no data", () => {
    cy.intercept("/timeline", { body: new Uint8Array([0x90]).buffer })

    cy.get("@button").click()

    cy.get("@error").should("include.text", "empty")
  })

  it("fails if color index is not in palette", () => {
    cy.intercept("/timeline", {
      body: new Uint8Array([
        0x92, 0x92, 0xce, 0x64, 0xa7, 0x37, 0xbc, 0x0f, 0x92, 0xce, 0x64, 0xa7,
        0xe0, 0x9c, 0x01,
      ]).buffer,
    })

    cy.get("@button").click()

    cy.get("@error").should("include.text", "index").and("include.text", "15")
  })

  it("renders according to snapshot", () => {
    cy.get("@canvas").should("not.have.class", "drawed")

    cy.get("@button").click()

    cy.get("@canvas").should("have.class", "drawed").matchImage()
  })
})
