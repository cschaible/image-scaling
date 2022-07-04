package simulations

import io.gatling.javaapi.core.*
import io.gatling.javaapi.core.CoreDsl.*
import io.gatling.javaapi.http.HttpDsl.*

class ImgProxyResizeImageSimulation : Simulation() {

  private val resizeImages =
      during(60)
          .on(
              exec(
                  http("Resize")
                      .get(
                          "http://localhost:3000/AfrOrF3gWeDA6VOlDG4TzxMv39O7MXnF4CXpKUwGqRM/resize:fit:1920:1920:0/gravity:sm/plain/http%3A%2F%2F172.17.0.1:3001/static/image.jpg")))

  private val httpProtocol =
      http.userAgentHeader(
          "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.8; rv:16.0) Gecko/20100101 Firefox/16.0")

  private val resizeImagesScenario = scenario("Resize images").exec(resizeImages)

  init {
    setUp(
            resizeImagesScenario.injectOpen(rampUsers(15).during(10)),
        )
        .protocols(httpProtocol)
  }
}
