describe("timeline", () => {
  beforeEach(() => {
    cy.visit("/")
    cy.dataCy("timeline-canvas").as("canvas")
    cy.dataCy("draw-button").as("button").should("have.class", "ready")
    cy.dataCy("error-out").as("error").should("be.empty")
  })

  it("fails on bad response status code", () => {
    cy.intercept("/api/influxdb*", { statusCode: 400 })

    cy.get("@button").click()

    cy.get("@error").should("include.text", "status").and("include.text", "400")
  })

  it("fails if there is no data", () => {
    cy.intercept("/api/influxdb*", { body: "" })

    cy.get("@button").click()

    cy.get("@error").should("include.text", "empty")
  })

  it("fails if first datapoint is not good", () => {
    cy.intercept("/api/influxdb*", {
      body: "_start,_stop\nzz,2022-12-09T14:58:09Z",
    })

    cy.get("@button").click()

    cy.get("@error")
      .should("include.text", "CSV")
      .and("include.text", "deserialize")
  })

  it("fails if record deserialization errors", () => {
    const body = ""
    cy.intercept("/api/influxdb*", {
      body:
        "_start,_stop,_time,color\n" +
        "2022-12-08T14:58:09Z,2022-12-09T14:58:09Z,zz,#f2c037\n",
    })

    cy.get("@button").click()

    cy.get("@error")
      .should("include.text", "CSV")
      .and("include.text", "deserialize")
  })

  it("fails if color parsing errors", () => {
    const body = ""
    cy.intercept("/api/influxdb*", {
      body:
        "_start,_stop,_time,color\n" +
        "2022-12-08T14:58:09Z,2022-12-09T14:58:09Z,2022-12-08T14:58:40Z,zz\n",
    })

    cy.get("@button").click()

    cy.get("@error")
      .should("include.text", "invalid")
      .and("include.text", "color")
  })

  it("renders according to snapshot", () => {
    cy.get("@canvas").should("not.have.class", "drawed")

    cy.get("@button").click()

    cy.get("@canvas").should("have.class", "drawed").matchImage()
  })
})
