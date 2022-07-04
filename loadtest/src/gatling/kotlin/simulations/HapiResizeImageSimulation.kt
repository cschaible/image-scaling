package simulations

import io.gatling.javaapi.core.*
import io.gatling.javaapi.core.CoreDsl.*
import io.gatling.javaapi.http.HttpDsl.*

class HapiResizeImageSimulation : Simulation() {

  private val resizeImages =
      during(60)
          .on(
              exec(
                  http("Resize")
                      .post("/")
                      .bodyPart(
                          RawFileBodyPart(
                              "file", "images/image.jpg"))))

  private val httpProtocol =
      http
          .baseUrl("http://localhost:3000")
          .userAgentHeader(
              "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.8; rv:16.0) Gecko/20100101 Firefox/16.0")

  private val resizeImagesScenario = scenario("Resize images").exec(resizeImages)

  init {
    setUp(
            resizeImagesScenario.injectOpen(rampUsers(10).during(10)),
        )
        .protocols(httpProtocol)
  }
}
